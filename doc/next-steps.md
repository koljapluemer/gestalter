# Next steps

The project's roadmap, and what the foundation still needs, is in the gestalts repo's `doc/next-steps.md`. These are
the Gestalter's own steps.

- [ ] **The desktop app serves the Gestalter.** The node serves the Gestalter's build on `127.0.0.1:7717` and the
  tray opens it in the default browser. The UI uses the folder through the node directly, with no OPFS layer to sync.
  The status window becomes a settings pane. Same origin removes CORS and Private Network Access for this app. A
  webview can come later: on Linux it would bring back GTK (webkit2gtk), which the tray avoids. This is also when the
  binary and package stop being called `gestalts-connector` (mind installed users' config, autostart and package
  identity).
- [ ] **Pairing**: the desktop shows a QR code with its address and key (the node does the rest).
- [ ] **Android.** First a spike: the core and one pane on a real phone, to see whether Tauri 2's Android support
  holds up. Then:
  - the UI: `@gestalts/ui` and the plugins in Android's WebView (Chrome)
  - the core native, with `FsStorage` in app-private storage; `@gestalts/client`'s Tauri IPC backend
  - Kotlin for background sync (WorkManager, every 15 minutes at best; at once while the app is open), QR scanning,
    the camera, sign-in tabs and mDNS
  - an Android distribution (learn, quotes, explorer, greek-alphabet; no canvas or Canvas2Typ), and "last synced"
    shown
- [ ] Release builds of the desktop app on macOS and Windows runners (their tray and event-loop code is untested),
  and code signing.
- [ ] CI that checks out gestalts next to this repo, until the foundation is published.
