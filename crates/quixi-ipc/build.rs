//! Embeds `reference/ipc-exact-schemas.json` when it exists. `/reference/` is gitignored
//! (it is generated from an extracted app bundle), so fresh clones fall back to `{}`.

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src = manifest_dir.join("../../reference/ipc-exact-schemas.json");
    println!("cargo:rerun-if-changed={}", src.display());

    let json = fs::read_to_string(&src).unwrap_or_else(|_| "{}".to_string());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ipc-exact-schemas.json");
    fs::write(out, json).unwrap();
}
