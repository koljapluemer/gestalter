//! gestalts-connector, the Gestalter's desktop app: the node (a folder on this computer as the source of truth for
//! the Gestalts web apps, and the host of their plugins' workers) with a tray icon, a status window where you
//! choose the folder, and the Gestalter's plugins built in.
//!
//! Started without a subcommand it is the desktop app; `serve`, `lint` and `publish` are the node's own.

use clap::{Parser, Subcommand};
use gestalts_node::{Embedded, NodeCommand, Opts};
use include_dir::{Dir, include_dir};

mod status;
mod tray;

pub const ICON: &[u8] = include_bytes!("../assets/icon.png");
pub const TITLE: &str = "Gestalts Connector";
/// Passed to the status window so it can use `/control/*`.
pub const TOKEN_ENV: &str = "GESTALTS_CONTROL_TOKEN";
/// The Gestalter's hosted frontends, allowed to connect out of the box.
pub const ORIGINS: &[&str] = &["https://gestalter.koljasam.com"];

/// The plugins the Gestalter ships (its `@gestalts-plugin/*` dependencies), copied by `build.rs`.
static BUILTIN: Dir<'static> = include_dir!("$OUT_DIR/plugins");
// TODO: publish the SDK to PyPI and depend on a version instead, which third-party plugins need anyway
static SDK: Dir<'static> = include_dir!("$OUT_DIR/sdk");

pub const EMBEDDED: Embedded = Embedded { builtin: Some((&BUILTIN, &SDK)), origins: ORIGINS };

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[command(flatten)]
    opts: Opts,
}

#[derive(Subcommand)]
enum Command {
    #[command(flatten)]
    Node(NodeCommand),
    /// the status window (opened from the tray)
    #[command(hide = true)]
    Status {
        /// open the folder picker right away
        #[arg(long)]
        choose: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        None => tray::run(&cli.opts),
        Some(Command::Status { choose }) => status::run(&cli.opts, choose),
        Some(Command::Node(command)) => gestalts_node::run_command(command, &cli.opts, EMBEDDED),
    };
    if let Err(e) = result {
        eprintln!("gestalts-connector: {e}");
        std::process::exit(1);
    }
}
