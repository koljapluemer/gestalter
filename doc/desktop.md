# The desktop app

`gestalts-connector` (`desktop/`) is the Gestalter's desktop app: the gestalts node, which makes a folder on this
computer the source of truth for the web apps and runs their plugins' workers, inside a tray. Install it from a
package (`mise run package` / `mise run install`, see the README), launch it from the app menu, and choose your
folder in its window. It remembers the folder and lives in the tray from then on.

The server, folders, sync, plugins, workers and access are the node's: see `doc/node.md` in the gestalts repo.

```sh
gestalts-connector                    # the desktop app: tray + status window, one instance per user
gestalts-connector --background       # same, without opening the window ("Start on login" uses this)
gestalts-connector serve [--data DIR] # headless; the folder from --data, else the remembered one
gestalts-connector lint DIR           # gestalts and lints per note
gestalts-connector publish DIR        # prepare a folder as a read-only deck
# all of them: [--port 7717] [--profile NAME] [--plugins-dir DIR]... [--allow-origin URL]...
```

In development, `mise run dev` starts it with `--profile dev --plugins-dir ../gestalts/plugins`. The dev profile keeps
its settings apart from an installed app. Both use port 7717, so quit one before starting the other.

## Tray and window

- **Tray:** shows the folder and has "Show status…", "Choose folder…" and "Quit". Clicking the icon opens the status
  window. On Linux the tray is a StatusNotifierItem over D-Bus, so there's no GTK dependency. Desktops without a tray
  (e.g. GNOME without an extension) still get the window.
- **Status window:** a separate `gestalts-connector status` process, so closing it never affects the server. It shows
  the folder (choose another one, open it), the web apps connected in the last minute, uv and each plugin's state,
  "Start on login", and "Quit connector". It talks to the server over `/control/*` with the per-run token.
- **One instance:** launching it again while it runs makes the running one show its window, then exits.

## What it brings along

- **Built-in plugins:** the Gestalter's, i.e. the `@gestalts-plugin/*` dependencies in `distribution/package.json`.
  `desktop/build.rs` copies their manifests, gestalts and workers (not their UI or decks), plus the Python SDK next
  to them, from where pnpm linked them, and the binary unpacks them on start. Run `mise run setup` before building.
- **Origins:** the Gestalter's hosted frontend, `https://gestalter.koljasam.com`, may connect out of the box.

## Packaging

`mise run package` builds this OS's installers into `target/packages` (Linux: `.deb`, `.rpm`, AppImage; macOS:
`.dmg`; Windows: setup `.exe`) with cargo-packager and cargo-generate-rpm (the `[package.metadata.*]` sections of
`desktop/Cargo.toml`). `mise run install` installs one here like a user would (`scripts/install-package.sh`). The
installed app needs nothing else: it brings its plugins, and downloads uv (and through it Python) for them when
needed.
