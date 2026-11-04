# The desktop app

`gestalter` (`desktop/`) is the Gestalter's desktop app: the gestalts node, which makes a folder on this computer the
source of truth and runs the plugins' workers, inside a tray, with the Gestalter's web build built in. The node serves
that build on `http://127.0.0.1:7717`, and the app opens it in the default browser. Install it from a package
(`mise run package` / `mise run install`, see the README) and launch it from the app menu. On first start the
Gestalter asks for your folder; the app remembers it and lives in the tray from then on.

The server, folders, change events, sync, plugins, workers and access are the node's: see `doc/node.md` in the
gestalts repo.

```sh
gestalter                    # the desktop app: tray, opens the Gestalter; one instance per user
gestalter --background       # same, without opening it ("Start on login" uses this; opens it anyway with no folder)
gestalter serve [--data DIR] # headless; the folder from --data, else the remembered one; still serves the Gestalter
gestalter lint DIR           # gestalts and lints per note
gestalter publish DIR        # prepare a folder as a read-only deck
# all of them: [--port 7717] [--profile NAME] [--plugins-dir DIR]... [--allow-origin URL]... [--app-dir DIR]
```

In development, `mise run dev` starts it with `--profile dev --plugins-dir ../gestalts/plugins --open-url
http://localhost:5173/`, next to Vite on :5173, which proxies the node's API (`distribution/vite.config.ts`). So the
dev page uses the folder directly as the served one does. The dev profile keeps its settings apart from an installed
app. Both use port 7717, so quit one before starting the other.

## The Gestalter on the folder

The page opens the notes on the node at its own origin (`usePluginRepo({ node })`): every write goes to the folder,
nothing is kept in the browser, and nothing syncs. The node's `/events` socket brings changes from elsewhere (workers,
loops-for-learning's sync, a hand edit) into the open page. Without the desktop app running, the page says so and
reconnects when it's back. Being same-origin with the node, it needs no CORS or Private Network Access.

## Tray and settings

- **Tray:** shows the folder and has "Open Gestalter" (also a left click), "Choose folder…" and "Quit". On Linux the
  tray is a StatusNotifierItem over D-Bus, so there's no GTK dependency. Desktops without a tray (e.g. GNOME without
  an extension) still get the Gestalter: launching the app again opens it.
- **Desktop settings** (the drive icon in the Gestalter's tab bar, `distribution/src/DesktopSettings.tsx`): the folder
  (choose another, show it in the file manager), uv and each plugin's state, other web apps connected in the last
  minute, conflicts, "Start on login" and "Quit desktop app". They use the node's `/control/*`, which the page may use
  because the node served it; the native parts (picker, file manager, autostart) are the tray process's `Shell`
  (`desktop/src/tray.rs`). Choosing another folder reloads the Gestalter.
- **One instance:** launching it again while it runs opens the Gestalter in the running one, then exits.

## What it brings along

- **The Gestalter:** `distribution/dist`, embedded by `desktop/build.rs`. A release build fails without it (`mise run
  package` builds it first); a debug build serves a placeholder page instead.
- **Built-in plugins:** the Gestalter's, i.e. the `@gestalts-plugin/*` dependencies in `distribution/package.json`.
  `desktop/build.rs` copies their manifests, gestalts and workers (not their UI or decks), plus the Python SDK next
  to them, from where pnpm linked them, and the binary unpacks them on start. Run `mise run setup` before building.
- **Origins** (`ORIGINS` in `desktop/src/main.rs`): other web frontends allowed to use the node. For now the hosted
  Gestalter's, `https://gestalter.koljasam.com`, so its users can sync their browser's notes into the folder one last
  time; loops-for-learning's once it is deployed.

## From `gestalts-connector`

The app used to be called `gestalts-connector` ("Gestalts Connector"). Its settings (the folder, `extra_origins`) are
the node's and stay. "Start on login" moves to the new name on first start. The `.rpm` obsoletes the old package; the
`.deb` can't say so, so remove `gestalts-connector` by hand. If the old app still runs, the new one says so and
exits. Users of the hosted Gestalter: run the new app, connect the hosted page to it once more (its Connector menu)
to sync, then use the Gestalter from the tray.

## Packaging

`mise run package` builds this OS's installers into `target/packages` (Linux: `.deb`, `.rpm`, AppImage; macOS:
`.dmg`; Windows: setup `.exe`) with cargo-packager and cargo-generate-rpm (the `[package.metadata.*]` sections of
`desktop/Cargo.toml`). `mise run install` installs one here like a user would (`scripts/install-package.sh`). The
installed app needs nothing else: it brings the Gestalter and its plugins, and downloads uv (and through it Python)
for them when needed.
