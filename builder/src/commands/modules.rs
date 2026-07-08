use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{cargo::CargoOpts, commands::TARGET};

pub fn build_all(target_dir: &Path, release: bool) -> Result<Vec<(String, PathBuf)>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let services_dir = manifest_dir.parent().unwrap().join("service");
    let linker_ld = services_dir.join("linker.ld");

    if !services_dir.exists() {
        return Ok(Vec::new());
    }

    let mut modules = Vec::new();
    for entry in std::fs::read_dir(&services_dir)
        .with_context(|| format!("Failed to read {}", services_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() || !path.join("Cargo.toml").exists() {
            continue;
        }

        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let module = build_one(&name, target_dir, release, &linker_ld)?;
        modules.push(module);
    }
    Ok(modules)
}

fn build_one(
    name: &str,
    target_dir: &Path,
    release: bool,
    linker_ld: &Path,
) -> Result<(String, PathBuf)> {
    let mut cargo = CargoOpts::new(name.into());
    cargo.target(TARGET.into());
    cargo.config("lib.crate-type = [\"staticlib\"]".into());
    cargo.env("RUSTFLAGS", "-C relocation-model=pic");
    if release {
        cargo.release();
    }
    cargo.done()?;

    let profile = if release { "release" } else { "debug" };
    let lib_a = target_dir
        .join(TARGET)
        .join(profile)
        .join(format!("lib{}.a", name.replace('-', "_")));
    let ko = target_dir
        .join(TARGET)
        .join(profile)
        .join(format!("{name}.ko"));

    let status = std::process::Command::new("lld")
        .arg("-flavor")
        .arg("gnu")
        .arg("-r")
        .arg("-o")
        .arg(&ko)
        .arg("--gc-sections")
        .arg("-T")
        .arg(&linker_ld)
        .arg(&lib_a)
        .status()?;
    anyhow::ensure!(status.success(), "rust-lld failed for {name}");

    println!("  module {name} -> {ko}", ko = ko.display());
    Ok((name.to_string(), ko))
}
