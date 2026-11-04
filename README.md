# Gestalter

A personal learning and note-taking app on [gestalts](../gestalts): the host with every plugin, panes as tabs
(`distribution/`), and its desktop app (`desktop/`, `gestalts-connector`), which keeps your notes in a folder on this
computer and runs the plugins' workers. `android/` and `ios/` come later. See [`doc/desktop.md`](doc/desktop.md) and
[`doc/next-steps.md`](doc/next-steps.md).

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
mise run dev          # desktop app (dev profile, gestalts' plugins) + Gestalter :5173, hot reload, gestalts included
mise run check        # tests, lints, typechecks
mise tasks            # everything else
```

On Linux, building the desktop app needs no GTK; its tray talks D-Bus directly.

## Shipping

- **Web frontend:** `mise run build` writes `distribution/dist`, static files to host anywhere. It is meant for
  `https://gestalter.koljasam.com`, which the desktop app allows out of the box.
- **Desktop app:** `mise run package` builds this OS's installers into `target/packages` (Linux: `.deb`, `.rpm`,
  AppImage; macOS: `.dmg`; Windows: setup `.exe`). `mise run install` builds them and installs one here like a user
  would. See [`doc/desktop.md`](doc/desktop.md).
