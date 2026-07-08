use std::process::{Command, Stdio};

use anyhow::Result;
use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};

use crate::{RunArgs, commands::do_build};

pub fn do_run(args: RunArgs) -> Result<()> {
    let RunArgs {
        build_args,
        cores,
        serial,
        debug,
    } = args;

    let (kernel_path, img_path) = do_build(build_args)?;
    println!("Kernel: {}", kernel_path.display());
    println!("Image:   {}", img_path.display());

    let mut cmd = Command::new("qemu-system-riscv64");
    cmd.arg("-machine").arg("virt");
    cmd.arg("-cpu").arg("rv64");
    cmd.arg("-m").arg("1G");
    cmd.arg("-smp").arg(cores.to_string());
    if serial {
        cmd.arg("-display").arg("none");
    } else {
        cmd.arg("-device").arg("ramfb");
        cmd.arg("-display").arg("sdl");
    }

    // 磁盘镜像（含 Limine EFI + kernel + limine.conf）
    cmd.arg("-drive").arg(format!(
        "file={},format=raw,if=none,id=drive0",
        img_path.display()
    ));
    cmd.arg("-device").arg("virtio-blk-device,drive=drive0");

    // OVMF / UEFI 固件
    let ovmf_path = Prebuilt::fetch(Source::LATEST, "target/ovmf")
        .expect("failed to fetch OVMF")
        .get_file(Arch::Riscv64, FileType::Code);
    cmd.arg("-drive")
        .arg(format!("if=pflash,format=raw,file={}", ovmf_path.display()));

    if serial {
        cmd.arg("-serial").arg("stdio");
    }

    if debug {
        cmd.arg("-s").arg("-S");
    }

    if !debug {
        let status = cmd.spawn()?.wait()?;
        anyhow::ensure!(status.success(), "QEMU exited with {status}");
    } else {
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        let mut qemu = cmd.spawn()?;

        let mut gdb = Command::new("gdb-multiarch");
        gdb.arg("-ex")
            .arg(format!("file {}", kernel_path.display()));
        gdb.arg("-ex").arg("target remote localhost:1234");
        let status = gdb.spawn()?.wait()?;
        anyhow::ensure!(status.success(), "GDB exited with {status}");

        qemu.kill()?;
    }

    Ok(())
}
