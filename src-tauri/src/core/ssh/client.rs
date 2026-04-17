use anyhow::Result;
use ssh2::Session as Ssh2Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use std::time::Duration;

pub struct SSHSession {
    session: Arc<Mutex<Option<Ssh2Session>>>,
    channel: Arc<Mutex<Option<ssh2::Channel>>>,
}

impl SSHSession {
    pub async fn new(
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        _key_path: Option<String>,
    ) -> Result<Self> {
        let (session, channel) = task::spawn_blocking(move || {
            // Connect to the SSH server
            let tcp = TcpStream::connect((host.as_str(), port))?;
            tcp.set_nodelay(true)?;
            // Use a longer timeout for reading
            tcp.set_read_timeout(Some(Duration::from_millis(100)))?;

            // Create SSH session
            let mut sess = Ssh2Session::new()?;
            sess.set_tcp_stream(tcp);
            sess.handshake()?;

            // Authenticate with password
            if let Some(password) = password {
                sess.userauth_password(&username, &password)?;
            } else {
                return Err(anyhow::anyhow!("Password authentication required but no password provided"));
            }

            if !sess.authenticated() {
                return Err(anyhow::anyhow!("SSH authentication failed"));
            }

            // Open a channel and request a PTY
            let mut channel = sess.channel_session()?;
            channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))?;
            channel.shell()?;

            Ok::<(Ssh2Session, ssh2::Channel), anyhow::Error>((sess, channel))
        }).await??;

        Ok(Self {
            session: Arc::new(Mutex::new(Some(session))),
            channel: Arc::new(Mutex::new(Some(channel))),
        })
    }

    pub async fn send(&self, data: &str) -> Result<()> {
        let channel = self.channel.clone();
        let data = data.to_string();

        task::spawn_blocking(move || {
            let mut ch_guard = channel.blocking_lock();
            if let Some(ref mut ch) = *ch_guard {
                ch.write_all(data.as_bytes())?;
                ch.flush()?;
            }
            Ok::<(), anyhow::Error>(())
        }).await?
    }

    pub async fn read(&self) -> Result<Option<Vec<u8>>> {
        let channel = self.channel.clone();

        task::spawn_blocking(move || {
            let mut ch_guard = channel.blocking_lock();
            if let Some(ref mut ch) = *ch_guard {
                let mut buffer = vec![0u8; 8192];

                match ch.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        buffer.truncate(n);
                        Ok(Some(buffer))
                    }
                    Ok(_) => {
                        // Ok(0) means EOF - connection closed
                        Ok(None)
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut => {
                        Ok(None)
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                Ok(None)
            }
        }).await?
    }

    pub async fn close(&self) -> Result<()> {
        let channel = self.channel.clone();
        let session = self.session.clone();

        task::spawn_blocking(move || {
            // Close channel
            {
                let mut ch_guard = channel.blocking_lock();
                if let Some(mut ch) = ch_guard.take() {
                    let _ = ch.close();
                    let _ = ch.wait_close();
                }
            }
            // Session will be closed when dropped
            let _ = session.blocking_lock().take();
            Ok::<(), anyhow::Error>(())
        }).await?
    }
}
