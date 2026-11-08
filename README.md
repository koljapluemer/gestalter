# gestalter

A personal learning and note-taking app on [gestalts](../gestalts): views as tabs (`distribution/`), with the note views
always there and others installable, and its desktop app (`desktop/`), which keeps your ymls in a folder on this computer,
serves gestalter on `127.0.0.1:7717` and runs plugin backends. The Android app (`android/`,
`distribution-android/`) keeps the ymls on the phone and syncs them with the paired desktop. `ios/` comes later. See
[`doc/desktop.md`](doc/desktop.md), [`doc/android.md`](doc/android.md) and [`doc/next-steps.md`](doc/next-steps.md).

## Developing

Needs [mise](https://mise.jdx.dev), [rustup](https://rustup.rs), and checkouts of gestalts and gestalt-plugins next
to this one: until they are published, this repo links their packages, plugins and node by path.

```
parent/
  gestalts/
  gestalt-plugins/
  gestalter/
```

```sh
mise install          # once after cloning, and when mise.toml changes
mise run dev          # desktop app (dev profile, both repos' plugins) + gestalter :5173 on it, hot reload
mise run check        # tests, lints, typechecks
mise run android-dev  # the Android app on a connected phone or emulator (needs the Android SDK and NDK)
mise tasks            # everything else
```

On Linux, building the desktop app needs no GTK; its tray talks D-Bus directly.

## Shipping

`mise run package` builds the gestalter (`mise run build`, into `distribution/dist`), embeds it in the desktop app
and builds this OS's installers into `target/packages` (Linux: `.deb`, `.rpm`, AppImage; macOS: `.dmg`; Windows:
setup `.exe`). `mise run install` builds them and installs one here like a user would. See
[`doc/desktop.md`](doc/desktop.md). `mise run android-build` builds a debug APK ([`doc/android.md`](doc/android.md)).
