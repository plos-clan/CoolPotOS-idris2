use std::path::PathBuf;

use anyhow::Result;

use crate::{
    BuildArgs,
    cargo::CargoOpts,
    commands::{TARGET, build_user_programs, kernel_path, modules, target_dir},
    image,
};

pub fn do_build(args: BuildArgs) -> Result<(PathBuf, PathBuf)> {
    let target_dir = target_dir();

    let BuildArgs { release } = args;

    let user_programs = build_user_programs(&target_dir, release)?;
    let built_modules = modules::build_all(&target_dir, release)?;

    let mut kernel = CargoOpts::new("kernel".into());
    kernel.target(TARGET.into());
    if release {
        kernel.release();
    }
    kernel.done()?;
    let kpath = kernel_path(release);

    image::build(&kpath, user_programs, built_modules).map(|p| (kpath, p))
}
