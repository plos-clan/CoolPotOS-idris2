use anyhow::Result;

use crate::{cargo::CargoOpts, commands::TARGET};

pub fn do_clippy() -> Result<()> {
    let mut kernel = CargoOpts::new("kernel".into());
    kernel.action("clippy");
    kernel.target(TARGET.into());
    kernel.done()
}
