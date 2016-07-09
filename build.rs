extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, env!("OUT_DIR"));
}
