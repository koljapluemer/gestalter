import { useEffect, useState } from 'react';
import { useConnector } from '@gestalts/client';
import { Conflicts } from '@gestalts/kit';
import { desktop, type DesktopStatus } from './desktop';

const STATE: Record<string, string> = { stopped: 'stopped', starting: 'starting…', ready: 'ready', running: 'running', error: 'error' };

/** The desktop app's settings: the folder, the plugins' workers, start on login. Refreshed while shown. */
export function DesktopSettings() {
  const connection = useConnector();
  const [status, setStatus] = useState<DesktopStatus>();
  const [error, setError] = useState<string>();
  useEffect(() => {
    let live = true;
    const poll = () => desktop.status().then((s) => { if (live) setStatus(s); }, () => undefined);
    void poll();
    const timer = setInterval(() => void poll(), 2000);
    return () => { live = false; clearInterval(timer); };
  }, []);
  const act = (f: () => Promise<DesktopStatus | void>) => {
    setError(undefined);
    f().then((s) => { if (s) setStatus(s); }, (e: unknown) => setError(e instanceof Error ? e.message : String(e)));
  };

  if (connection.state === 'error') {
    return <p className="w-72 text-sm text-error">{connection.error} Start it again; this page picks up where it was.</p>;
  }
  if (!status) return <span className="loading loading-spinner loading-sm" />;
  return (
    <div className="flex w-80 flex-col gap-3 text-sm">
      <section className="flex flex-col gap-1">
        <h3 className="font-semibold">Folder</h3>
        <code className="block truncate text-xs" title={status.folder?.path}>{status.folder?.path ?? 'none'}</code>
        <div className="flex gap-2">
          <button type="button" className="btn btn-sm" onClick={() => act(desktop.chooseFolder)}>Choose another folder…</button>
          <button type="button" className="btn btn-sm btn-ghost" onClick={() => act(desktop.reveal)}>Show</button>
        </div>
        <p className="text-xs text-base-content/60">Choosing another folder reloads the Gestalter with its notes.</p>
      </section>

      <section className="flex flex-col gap-1">
        <h3 className="font-semibold">Plugins</h3>
        <p className="text-xs text-base-content/60">
          {status.uv.state === 'locating' && 'Looking for uv…'}
          {status.uv.state === 'downloading' && 'Downloading uv, which runs the plugins…'}
          {status.uv.state === 'ready' && `uv ${status.uv.version}`}
          {status.uv.state === 'failed' && <span className="text-error">uv: {status.uv.error}</span>}
        </p>
        {status.plugins.length === 0 && <p>No plugins with workers.</p>}
        {status.plugins.map((p) => (
          <div key={p.id}>
            {p.id} {p.version} · {STATE[p.state] ?? p.state}
            {p.error && <p className="text-xs text-error">{p.error}</p>}
          </div>
        ))}
      </section>

      {status.clients.length > 0 && (
        <section className="flex flex-col gap-1">
          <h3 className="font-semibold">Other web apps</h3>
          {status.clients.map((c) => <div key={c.origin} className="truncate text-xs">{c.origin} · {c.seconds_ago} s ago</div>)}
        </section>
      )}

      <Conflicts />

      <div className="flex items-center gap-2 border-t border-base-300 pt-2">
        {status.autostart !== null && (
          <label className="label flex-1 cursor-pointer gap-2">
            <input type="checkbox" className="checkbox checkbox-sm" checked={status.autostart}
              onChange={(e) => act(() => desktop.setAutostart(e.target.checked))} />
            Start on login
          </label>
        )}
        <button type="button" className="btn btn-sm btn-ghost ml-auto" onClick={() => act(desktop.quit)}>Quit desktop app</button>
      </div>
      {error && <p className="text-xs text-error">{error}</p>}
      <p className="text-xs text-base-content/50">v{status.version} · {status.address}</p>
    </div>
  );
}
