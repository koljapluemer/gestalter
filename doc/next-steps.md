# Next steps

The project's roadmap, and what the foundation still needs, is in the gestalts repo's `doc/next-steps.md`. These are
the Gestalter's own steps.

- [x] **The desktop app serves the Gestalter**, as `gestalter` (formerly `gestalts-connector`): see
  [desktop.md](desktop.md).
- [ ] Take down the hosted `gestalter.koljasam.com` once its users had a release to sync into their folder, then drop
  it from `ORIGINS` (`desktop/src/main.rs`).
- [ ] A webview instead of the default browser, maybe: on Linux it would bring back GTK (webkit2gtk), which the tray
  avoids.
- [x] **Pairing**: the desktop's settings show a QR code ("Devices"), list the paired devices and forget them; the
  node does the rest (gestalts' `doc/node.md#pairing-and-the-home-network`).
- [ ] **Android.** First a spike: the core and one pane on a real phone, to see whether Tauri 2's Android support
  holds up. Then:
  - the UI: `@gestalts/ui` and the plugins in Android's WebView (Chrome)
  - the core native, with `FsStorage` in app-private storage; `@gestalts/client`'s Tauri IPC backend
  - pairing with `gestalts_peer::paired`: scan the code, `pair`, keep the `PairedNode` (its token privately), then
    `connect` and sync with a base per `base_name(node_id, folder_id)`; mDNS needs a `MulticastLock` meanwhile
  - Kotlin for background sync (WorkManager, every 15 minutes at best; at once while the app is open), QR scanning,
    the camera, sign-in tabs and mDNS
  - an Android distribution, `id: gestalter-android` (learn, quotes, explorer, alphabet-pronunciation-via-visual-cognates, learn-world-map; no canvas or
    Canvas2Typ), with tabs of its own, and "last synced" shown
- [ ] Release builds of the desktop app on macOS and Windows runners (their tray and event-loop code is untested),
  and code signing.
- [ ] CI that checks out gestalts and gestalt-plugins next to this repo, until they are published.
