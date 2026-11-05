//! The Gestalter on Android (`doc/android.md`): the Gestalter's page in the system WebView, its core in wasm over
//! the notes in this app's private storage (the commands below), and `gestalts_device` keeping that folder, the
//! pairing with the desktop, and the sync, telling the page what changed (the events).

use gestalts_device::{Event, Handle, Host, NodeInfo, Status};
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::{AppHandle, Emitter, Manager, State};

#[cfg(target_os = "android")]
mod android;

/// The events `@gestalts/client/tauri` listens to.
const CHANGED_EVENT: &str = "gestalts://changed";
const STATUS_EVENT: &str = "gestalts://status";

struct App {
    handle: AppHandle,
}

impl Host for App {
    fn event(&self, event: Event) {
        let sent = match event {
            Event::Changed { notes, blobs } => self.handle.emit(CHANGED_EVENT, serde_json::json!({ "notes": notes, "blobs": blobs })),
            Event::Status(status) => self.handle.emit(STATUS_EVENT, status),
        };
        if let Err(e) = sent {
            eprintln!("gestalter: telling the page failed: {e}");
        }
    }

    fn multicast(&self, hold: bool) {
        #[cfg(target_os = "android")]
        android::multicast(hold);
        #[cfg(not(target_os = "android"))]
        let _ = hold;
    }
}

type Device<'a> = State<'a, Handle>;
type Res<T> = Result<T, String>;

fn e(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[tauri::command]
async fn notes_list(d: Device<'_>) -> Res<Vec<String>> {
    d.list_notes().await.map_err(e)
}

#[tauri::command]
async fn note_read(d: Device<'_>, key: String) -> Res<Option<String>> {
    d.read_note(key).await.map_err(e)
}

#[tauri::command]
async fn notes_read(d: Device<'_>, keys: Vec<String>) -> Res<Vec<(String, String)>> {
    d.read_notes(keys).await.map_err(e)
}

#[tauri::command]
async fn notes_read_all(d: Device<'_>) -> Res<Vec<(String, String)>> {
    d.read_all_notes().await.map_err(e)
}

#[tauri::command]
async fn notes_hashes(d: Device<'_>) -> Res<std::collections::BTreeMap<String, String>> {
    d.note_hashes().await.map_err(e)
}

#[tauri::command]
async fn note_write(d: Device<'_>, key: String, text: String) -> Res<()> {
    d.write_note(key, text).await.map_err(e)
}

#[tauri::command]
async fn notes_write(d: Device<'_>, notes: Vec<(String, String)>) -> Res<()> {
    d.write_notes(notes).await.map_err(e)
}

#[tauri::command]
async fn note_delete(d: Device<'_>, key: String) -> Res<()> {
    d.delete_note(key).await.map_err(e)
}

#[tauri::command]
async fn blobs_list(d: Device<'_>) -> Res<Vec<String>> {
    d.list_blobs().await.map_err(e)
}

/// The blob's bytes after a byte that says whether it exists (1) or not (0).
#[tauri::command]
async fn blob_read(d: Device<'_>, name: String) -> Res<Response> {
    let out = match d.read_blob(name).await.map_err(e)? {
        Some(b) => [&[1u8][..], &b].concat(),
        None => vec![0],
    };
    Ok(Response::new(out))
}

/// The bytes are the request's body, the name its `x-blob-name` header (URI-encoded).
#[tauri::command]
async fn blob_write(d: Device<'_>, request: Request<'_>) -> Res<()> {
    let InvokeBody::Raw(bytes) = request.body() else { return Err("expected the blob's bytes".into()) };
    let name = request.headers().get("x-blob-name").and_then(|v| v.to_str().ok()).ok_or("expected the blob's name")?;
    let name = percent_encoding::percent_decode_str(name).decode_utf8().map_err(e)?.into_owned();
    d.write_blob(name, bytes.clone()).await.map_err(e)
}

#[tauri::command]
async fn blob_delete(d: Device<'_>, name: String) -> Res<()> {
    d.delete_blob(name).await.map_err(e)
}

#[tauri::command]
async fn gestalts_set(d: Device<'_>, yaml: Vec<String>) -> Res<()> {
    d.set_gestalts(yaml).await.map_err(e)
}

#[tauri::command]
async fn sync_status(d: Device<'_>) -> Res<Status> {
    d.status().await.map_err(e)
}

#[tauri::command]
async fn sync_now(d: Device<'_>) -> Res<()> {
    d.sync_soon().await.map_err(e)
}

#[tauri::command]
async fn set_active(d: Device<'_>, active: bool) -> Res<()> {
    d.set_active(active).await.map_err(e)
}

/// Pairs with the computer whose pairing code this is, under `device_name` or this phone's model.
#[tauri::command]
async fn pair(d: Device<'_>, uri: String, device_name: Option<String>) -> Res<NodeInfo> {
    let name = device_name.filter(|n| !n.trim().is_empty()).unwrap_or_else(default_device_name);
    d.pair(uri, name).await.map_err(e)
}

#[tauri::command]
async fn forget(d: Device<'_>) -> Res<()> {
    d.forget().await.map_err(e)
}

fn default_device_name() -> String {
    #[cfg(target_os = "android")]
    if let Some(name) = android::model() {
        return name;
    }
    "Android phone".into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_barcode_scanner::init())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            let device = gestalts_device::spawn(dir, App { handle: app.handle().clone() })?;
            app.manage(device);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            notes_list, note_read, notes_read, notes_read_all, notes_hashes, note_write, notes_write, note_delete,
            blobs_list, blob_read, blob_write, blob_delete, gestalts_set, sync_status, sync_now, set_active, pair, forget,
        ])
        .run(tauri::generate_context!())
        .expect("the Gestalter failed to start");
}
