# Gestalter

A personal learning and note-taking app on [gestalts](../gestalts): the host with every plugin, panes as tabs
(`distribution/`), and its desktop app (`desktop/`, `gestalter`), which keeps your notes in a folder on this computer,
serves the Gestalter on `127.0.0.1:7717` and runs the plugins' workers. `android/` and `ios/` come later. See
[`doc/desktop.md`](doc/desktop.md) and [`doc/next-steps.md`](doc/next-steps.md).

## Developing

Needs [mise](https://mise.jdx.dev), [rustup](https://rustup.rs), and a checkout of gestalts next to this one: until
it is published, this repo links its packages, plugins and node by path.

```
parent/
  gestalts/
  gestalter/
```

```sh
mise install          # once after cloning, and when mise.toml changes
mise run dev          # desktop app (dev profile, gestalts' plugins) + Gestalter :5173 on it, hot reload
mise run check        # tests, lints, typechecks
mise tasks            # everything else
```

On Linux, building the desktop app needs no GTK; its tray talks D-Bus directly.

## Shipping

`mise run package` builds the Gestalter (`mise run build`, into `distribution/dist`), embeds it in the desktop app
and builds this OS's installers into `target/packages` (Linux: `.deb`, `.rpm`, AppImage; macOS: `.dmg`; Windows:
setup `.exe`). `mise run install` builds them and installs one here like a user would. See
[`doc/desktop.md`](doc/desktop.md).
