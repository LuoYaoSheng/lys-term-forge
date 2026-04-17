use anyhow::Result;
use ssh2::Session as Ssh2Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::fs;
use tauri::{AppHandle, Emitter};
use tracing::{info, warn};

use crate::models::events::AppEvent;

/// Known hosts file path: ~/.termforge/known_hosts
fn known_hosts_path() -> Result<std::path::PathBuf> {
    let mut path = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    path.push(".termforge");
    fs::create_dir_all(&path)?;
    path.push("known_hosts");
    Ok(path)
}

/// Compute a hex SHA-256 fingerprint of the remote host key.
fn host_key_fingerprint(sess: &Ssh2Session) -> Result<String> {
    let (key, _) = sess.host_key()
        .ok_or_else(|| anyhow::anyhow!("No host key available after handshake"))?;
    use std::fmt::Write;
    let mut fingerprint = String::with_capacity(key.len() * 3);
    for (i, byte) in key.iter().enumerate() {
        if i > 0 { write!(fingerprint, ":")?; }
        write!(fingerprint, "{:02x}", byte)?;
    }
    Ok(fingerprint)
}

/// Verify the remote host key against the stored known_hosts file.
///
/// - First connection: record the key fingerprint and trust it.
/// - Subsequent connections: verify the key matches.
fn verify_host_key(sess: &Ssh2Session, host: &str, port: u16) -> Result<()> {
    let fingerprint = host_key_fingerprint(sess)?;
    let known_hosts = known_hosts_path()?;
    let entry = format!("{}:{} {}", host, port, fingerprint);

    if known_hosts.exists() {
        let content = fs::read_to_string(&known_hosts)?;
        for line in content.lines() {
            if let Some(stored) = line.strip_prefix(&format!("{}:{} ", host, port)) {
                if stored == fingerprint {
                    info!(host = %host, port = port, "Host key verified (known)");
                    return Ok(());
                } else {
                    warn!(host = %host, port = port, expected = %stored, got = %fingerprint,
                          "Host key mismatch — possible MITM attack");
                    return Err(anyhow::anyhow!(
                        "Host key mismatch for {}:{} — the server's fingerprint has changed. \
                         This may indicate a man-in-the-middle attack. \
                         Remove the old entry from ~/.termforge/known_hosts if this is expected.",
                        host, port
                    ));
                }
            }
        }
    }

    // First connection — trust on first use
    info!(host = %host, port = port, fingerprint = %fingerprint, "First connection — recording host key");
    let mut content = if known_hosts.exists() {
        fs::read_to_string(&known_hosts)?
    } else {
        String::new()
    };
    content.push_str(&entry);
    content.push('\n');
    fs::write(&known_hosts, content)?;

    // Set file permissions to owner-only on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        fs::set_permissions(&known_hosts, perms)?;
    }

    Ok(())
}

/// Commands sent to the SSH I/O thread via mpsc channel.
enum IoCommand {
    Write(Vec<u8>),
    Resize(u16, u16),
    Close,
}

/// SSH session backed by a dedicated OS thread for I/O.
///
/// Architecture:
/// - One dedicated OS thread owns the ssh2::Channel exclusively (zero lock contention)
/// - Reads: non-blocking mode, 5ms polling interval
/// - Writes: brief toggle to blocking mode for reliable delivery
/// - Communication via std::sync::mpsc (lock-free, instant send)
pub struct SSHSession {
    cmd_tx: std::sync::Mutex<std::sync::mpsc::Sender<IoCommand>>,
}

impl SSHSession {
    /// Create a new SSH session.
    ///
    /// This is a **blocking** call (TCP connect + handshake + auth).
    /// Must be called from `spawn_blocking`.
    ///
    /// On success, a background I/O thread starts reading from the channel
    /// and emitting `AppEvent`s to the frontend.
    pub fn new(
        app: AppHandle,
        session_id: String,
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        key_path: Option<String>,
    ) -> Result<Self> {
        // ── Connect + authenticate (blocking) ──
        let tcp = TcpStream::connect((host.as_str(), port))?;
        tcp.set_nodelay(true)?;
        tcp.set_read_timeout(Some(Duration::from_secs(30)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(10)))?;

        let mut sess = Ssh2Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        // ── Host key verification ──
        verify_host_key(&sess, &host, port)?;

        if let Some(password) = password {
            sess.userauth_password(&username, &password)?;
        } else if let Some(kp) = key_path {
            // Explicit key file path provided
            let path = std::path::Path::new(&kp);
            sess.userauth_pubkey_file(&username, None, path, None)?;
            info!(key = %kp, "Authenticated with specified public key");
        } else {
            // Try public key authentication with default SSH agent / ~/.ssh keys
            let mut authenticated = false;

            // Try default agent first
            if sess.authenticated() {
                authenticated = true;
            }

            // Try common key paths
            if !authenticated {
                if let Some(home) = dirs::home_dir() {
                    let key_paths = [
                        home.join(".ssh").join("id_ed25519"),
                        home.join(".ssh").join("id_rsa"),
                        home.join(".ssh").join("id_ecdsa"),
                    ];
                    for key_path in &key_paths {
                        if key_path.exists() {
                            match sess.userauth_pubkey_file(
                                &username,
                                None,
                                key_path,
                                None,
                            ) {
                                Ok(_) => {
                                    info!(key = %key_path.display(), "Authenticated with public key");
                                    authenticated = true;
                                    break;
                                }
                                Err(e) => {
                                    warn!(key = %key_path.display(), error = %e, "Public key auth failed, trying next");
                                }
                            }
                        }
                    }
                }
            }

            if !authenticated {
                return Err(anyhow::anyhow!(
                    "Authentication failed: no password provided and no suitable SSH key found"
                ));
            }
        }

        if !sess.authenticated() {
            return Err(anyhow::anyhow!("SSH authentication failed"));
        }

        let mut channel = sess.channel_session()?;
        channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))?;
        channel.shell()?;

        // Non-blocking mode for the polling read loop
        sess.set_blocking(false);

        info!(session_id = %session_id, host = %host, port = port, username = %username, "SSH session established");

        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<IoCommand>();

        // ── Spawn dedicated I/O thread ──
        std::thread::Builder::new()
            .name(format!("ssh-io-{}", &session_id[..12.min(session_id.len())]))
            .spawn(move || {
                let mut buf = vec![0u8; 8192];
                let sid = &session_id;

                loop {
                    // ── 1. Non-blocking read ──
                    match channel.read(&mut buf) {
                        Ok(0) => {
                            // EOF — remote closed
                            info!(session_id = %sid, "SSH connection closed by remote");
                            let _ = app.emit(
                                "app_event",
                                AppEvent::TerminalStatus {
                                    session_id: sid.clone(),
                                    status: "closed".into(),
                                    msg: Some("Connection closed by remote".into()),
                                },
                            );
                            break;
                        }
                        Ok(n) => {
                            let chunk = String::from_utf8_lossy(&buf[..n]);
                            let _ = app.emit(
                                "app_event",
                                AppEvent::TerminalData {
                                    session_id: sid.clone(),
                                    chunk: chunk.into_owned(),
                                },
                            );
                        }
                        Err(ref e)
                            if e.kind() == std::io::ErrorKind::WouldBlock
                                || e.kind() == std::io::ErrorKind::TimedOut =>
                        {
                            // No data — expected in non-blocking mode
                        }
                        Err(e) => {
                            warn!(session_id = %sid, error = %e, "SSH read error");
                            let _ = app.emit(
                                "app_event",
                                AppEvent::TerminalStatus {
                                    session_id: sid.clone(),
                                    status: "error".into(),
                                    msg: Some(format!("Read error: {}", e)),
                                },
                            );
                            break;
                        }
                    }

                    // ── 2. Process pending write commands ──
                    loop {
                        match cmd_rx.try_recv() {
                            Ok(IoCommand::Write(data)) => {
                                // Toggle to blocking for reliable write
                                sess.set_blocking(true);
                                let write_result =
                                    channel.write_all(&data).and_then(|_| channel.flush());
                                sess.set_blocking(false);

                                if let Err(e) = write_result {
                                    let _ = app.emit(
                                        "app_event",
                                        AppEvent::TerminalStatus {
                                            session_id: sid.clone(),
                                            status: "error".into(),
                                            msg: Some(format!("Write error: {}", e)),
                                        },
                                    );
                                    // Don't break — the connection might still be readable
                                }
                            }
                            Ok(IoCommand::Resize(cols, rows)) => {
                                sess.set_blocking(true);
                                if let Err(e) = channel.request_pty_size(cols as u32, rows as u32, None, None) {
                                    let _ = app.emit(
                                        "app_event",
                                        AppEvent::TerminalStatus {
                                            session_id: sid.clone(),
                                            status: "error".into(),
                                            msg: Some(format!("Resize error: {}", e)),
                                        },
                                    );
                                }
                                sess.set_blocking(false);
                            }
                            Ok(IoCommand::Close) => {
                                sess.set_blocking(true);
                                let _ = channel.close();
                                let _ = channel.wait_close();
                                let _ = app.emit(
                                    "app_event",
                                    AppEvent::TerminalStatus {
                                        session_id: sid.clone(),
                                        status: "closed".into(),
                                        msg: Some("Session closed".into()),
                                    },
                                );
                                return;
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => break,
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                // Sender dropped (SSHSession destroyed) — clean exit
                                sess.set_blocking(true);
                                let _ = channel.close();
                                return;
                            }
                        }
                    }

                    // ── 3. Yield CPU ──
                    std::thread::sleep(Duration::from_millis(5));
                }

                // Thread exits: session + channel dropped automatically
            })?;

        Ok(Self {
            cmd_tx: std::sync::Mutex::new(cmd_tx),
        })
    }

    /// Queue user input for writing to the SSH channel.
    /// Returns immediately; actual write happens on the I/O thread within ~5ms.
    pub fn send(&self, data: &str) -> Result<()> {
        self.cmd_tx
            .lock()
            .unwrap()
            .send(IoCommand::Write(data.as_bytes().to_vec()))
            .map_err(|_| anyhow::anyhow!("SSH session closed"))?;
        Ok(())
    }

    /// Request graceful shutdown.
    pub fn close(&self) -> Result<()> {
        self.cmd_tx
            .lock()
            .unwrap()
            .send(IoCommand::Close)
            .map_err(|_| anyhow::anyhow!("SSH session closed"))?;
        Ok(())
    }

    /// Resize the remote PTY dimensions.
    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        self.cmd_tx
            .lock()
            .unwrap()
            .send(IoCommand::Resize(cols, rows))
            .map_err(|_| anyhow::anyhow!("SSH session closed"))?;
        Ok(())
    }
}
