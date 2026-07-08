use std::path::{Path, PathBuf};

use anyhow::Result;
pub use build::*;
pub use clippy::*;
pub use run::*;
pub use test::*;

use crate::cargo::CargoOpts;

mod build;
mod clippy;
mod modules;
mod run;
mod test;

const TARGET: &str = "riscv64gc-unknown-none-elf";

fn kernel_path(release: bool) -> PathBuf {
    target_dir()
        .join(TARGET)
        .join(if release { "release" } else { "debug" })
        .join("kernel")
}

fn target_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
}

static USER_PROGRAMS: &[&str] = &[];

fn build_user_programs(target_dir: &Path, release: bool) -> Result<Vec<(String, PathBuf)>> {
    let build_one = |name: String| -> Result<_> {
        let mut cargo = CargoOpts::new(name.clone());
        cargo.target(TARGET.into());
        if release {
            cargo.release();
        }
        cargo.done()?;
        let path = target_dir
            .join(TARGET)
            .join(if release { "release" } else { "debug" })
            .join(&name);
        Ok((name.to_string(), path))
    };

    let mut result = Vec::new();
    for program in USER_PROGRAMS {
        let program = program.to_string();
        result.push(build_one(program)?);
    }
    Ok(result)
}
