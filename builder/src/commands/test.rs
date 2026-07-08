use anyhow::Result;

use crate::cargo::CargoOpts;

pub fn do_test() -> Result<()> {
    let mut kernel = CargoOpts::new("kernel".into());
    kernel.action("test");
    kernel.done()
}
