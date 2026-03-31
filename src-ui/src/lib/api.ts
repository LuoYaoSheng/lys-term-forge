import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export type AppEvent =
  | { type: 'terminal:data'; session_id: string; chunk: string }
  | { type: 'terminal:status'; session_id: string; status: 'connected' | 'reconnecting' | 'closed'; msg?: string }
  | { type: 'sftp:progress'; task_id: string; done: number; total: number }
  | { type: 'runbook:progress'; run_id: string; host_id: string; status: 'running' | 'ok' | 'fail'; tail?: string }
  | { type: 'monitor:snapshot'; host_id: string; ts: number; cpu: number; mem: number; disk: number; net_in: number; net_out: number };

export async function onAppEvent(cb: (e: AppEvent) => void): Promise<() => void> {
  try {
    const appWindow = getCurrentWindow();
    const unlisten = await appWindow.listen<AppEvent>('app_event', (event) => cb(event.payload));
    return () => unlisten();
  } catch (e) {
    console.error('Failed to setup event listener:', e);
    // Return a no-op function
    return () => {};
  }
}

export async function sessionOpen(req: {
  host: string;
  port: number;
  username: string;
  password?: string;
  mode: 'fake' | 'ssh';
}): Promise<{ session_id: string }> {
  return invoke('session_open', { req });
}

export async function sessionSend(req: { session_id: string; data: string }): Promise<void> {
  return invoke('session_send', { req });
}

export async function sessionClose(req: { session_id: string }): Promise<void> {
  return invoke('session_close', { req });
}

export async function sessionList(): Promise<
  Array<{ session_id: string; host: string; username: string; status: string }>
> {
  return invoke('session_list');
}

// Connection store types
export interface SavedConnection {
  id: string;
  name: string;
  mode: 'fake' | 'ssh';
  host: string;
  port: number;
  username: string;
  password?: string;
}

export async function connectionList(): Promise<SavedConnection[]> {
  return invoke('connection_list');
}

export async function connectionSave(conn: SavedConnection): Promise<void> {
  return invoke('connection_save', { conn });
}

export async function connectionDelete(id: string): Promise<void> {
  return invoke('connection_delete', { id });
}
