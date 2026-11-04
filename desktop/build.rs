//! Copies what the node needs of the Gestalter's plugins (the `@gestalts-plugin/*` dependencies of its web app:
//! manifests, gestalts, workers; not their UI or decks, and never `node_modules`) to `$OUT_DIR/plugins`, and the
//! Python SDK their workers use to `$OUT_DIR/sdk`, and the web app's build (`distribution/dist`) to `$OUT_DIR/app`,
//! which `include_dir!` embeds.
//!
//! Plugins are found where pnpm installed them for the web app, so run `mise run setup` first. A release build needs
//! the web app built (`mise run build`); a debug build without it serves a page saying so (in development the
//! Gestalter comes from Vite anyway).

use std::fs;
use std::path::{Path, PathBuf};

/// The web app whose plugins the desktop app ships.
const APP: &str = "../distribution";

fn main() {
    let app = Path::new(APP);
    let manifest = app.join("package.json");
    println!("cargo:rerun-if-changed={}", manifest.display());
    let pkg: serde_json::Value = serde_json::from_str(&fs::read_to_string(&manifest).expect("the web app has a package.json")).expect("package.json is JSON");
    let ids: Vec<&str> = pkg["dependencies"].as_object().into_iter().flatten()
        .filter_map(|(name, _)| name.strip_prefix("@gestalts-plugin/")).collect();

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let (plugins_out, sdk_out, app_out) = (out.join("plugins"), out.join("sdk"), out.join("app"));
    for dir in [&plugins_out, &sdk_out, &app_out] {
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
    }
    let mut sdk: Option<PathBuf> = None;
    for id in ids {
        let link = app.join("node_modules/@gestalts-plugin").join(id);
        let src = fs::canonicalize(&link).unwrap_or_else(|_| panic!("{} is missing; run `mise run setup`", link.display()));
        assert!(src.join("plugin.yml").is_file(), "{} has no plugin.yml", src.display());
        // include_dir! doesn't track changes by itself
        println!("cargo:rerun-if-changed={}", src.display());
        let dest = plugins_out.join(id);
        fs::create_dir_all(&dest).unwrap();
        fs::copy(src.join("plugin.yml"), dest.join("plugin.yml")).unwrap();
        for sub in ["gestalts", "worker"] {
            if src.join(sub).is_dir() {
                copy_tree(&src.join(sub), &dest.join(sub));
            }
        }
        // workers depend on the SDK by relative path (`../../../sdk/python`), next to `plugins/`
        if src.join("worker").is_dir() && sdk.is_none() {
            sdk = Some(fs::canonicalize(src.join("../../sdk/python")).expect("the SDK next to the plugins"));
        }
    }
    if let Some(sdk) = sdk {
        println!("cargo:rerun-if-changed={}", sdk.display());
        copy_tree(&sdk, &sdk_out);
    }

    let dist = app.join("dist");
    println!("cargo:rerun-if-changed={}", dist.display());
    if dist.join("index.html").is_file() {
        copy_tree(&dist, &app_out);
    } else if std::env::var("PROFILE").as_deref() == Ok("release") {
        panic!("{} is missing; run `mise run build` first", dist.join("index.html").display());
    } else {
        fs::write(app_out.join("index.html"), "<!doctype html><meta charset=utf-8><title>Gestalter</title>\
            <p>This debug build has no Gestalter built in. Run <code>mise run build</code>, or use the dev server \
            (<code>mise run dev</code>).</p>\n").unwrap();
    }
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap().flatten() {
        let name = entry.file_name();
        let name_s = name.to_string_lossy();
        if name_s.starts_with('.') || name_s == "__pycache__" || name_s == "node_modules" {
            continue;
        }
        let ty = entry.file_type().unwrap();
        if ty.is_dir() {
            copy_tree(&entry.path(), &to.join(&name));
        } else if ty.is_file() {
            fs::copy(entry.path(), to.join(&name)).unwrap();
        }
    }
}
