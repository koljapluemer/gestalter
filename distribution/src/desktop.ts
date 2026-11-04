/**
 * The desktop app this page is served by, through the node's `/control/*`: same-origin, so no token is needed.
 */

export type PluginStatus = { id: string; version: string; state: 'stopped' | 'starting' | 'ready' | 'running' | 'error'; error?: string | null };

export type DesktopStatus = {
  version: string;
  address: string;
  folder: { id: string; path: string } | null;
  clients: { origin: string; seconds_ago: number }[];
  plugins: PluginStatus[];
  uv: { state: 'locating' | 'downloading' } | { state: 'ready'; version: string } | { state: 'failed'; error: string };
  /** null: starting on login isn't available */
  autostart: boolean | null;
};

/** The node's own address: this page's origin (in development, Vite proxies it). */
export const NODE = location.origin;

async function control<T>(path: string, init?: RequestInit): Promise<T> {
  const r = await fetch(`${NODE}/control/${path}`, init);
  if (!r.ok) throw new Error((await r.text()) || `${r.status} ${r.statusText}`);
  return (r.status === 204 ? undefined : await r.json()) as T;
}

const json = (body: unknown): RequestInit => ({ method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });

export const desktop = {
  status: () => control<DesktopStatus>('status'),
  /** the native folder picker; the status afterwards, whether or not one was picked */
  chooseFolder: () => control<DesktopStatus>('choose-folder', { method: 'POST' }),
  reveal: () => control<void>('reveal', { method: 'POST' }),
  setAutostart: (on: boolean) => control<DesktopStatus>('autostart', json({ on })),
  quit: () => control<void>('quit', { method: 'POST' }),
};
