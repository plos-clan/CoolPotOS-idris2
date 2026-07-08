use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("missing OUT_DIR"));
    let target = env::var("TARGET").expect("missing TARGET");

    let idris_dir = manifest_dir.join("idris");
    let idris_src_dir = idris_dir.join("src");
    let idris_pkg_manifest = idris_dir.join("kernel.ipkg");
    let include_dir = manifest_dir.join("include");
    let idris_build_dir = out_dir.join("idris-build");
    let idris_output_dir = idris_build_dir.join("exec");
    let generated_pkg = out_dir.join("kernel.generated.ipkg");
    let generated_obj = out_dir.join("kernel_main.o");
    let generated_archive = out_dir.join("libkernel_idris_entry.a");

    emit_rerun_if_changed_tree(&idris_dir, &["idr", "lidr", "ipkg"]);
    emit_rerun_if_changed_tree(&include_dir, &["h", "hpp", "inc"]);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=IDRIS2");
    println!("cargo:rerun-if-env-changed=CLANG");
    println!("cargo:rerun-if-env-changed=LLVM_OBJCOPY");
    println!("cargo:rerun-if-env-changed=LLVM_AR");

    fs::create_dir_all(&idris_output_dir).expect("create idris output directory");

    let package_manifest =
        fs::read_to_string(&idris_pkg_manifest).expect("read Idris package manifest");
    let executable_name = extract_ipkg_field(&package_manifest, "executable")
        .expect("Idris package manifest must declare executable = ...");
    let rendered_package = render_build_ipkg(
        &package_manifest,
        &[
            ("sourcedir", &quote_ipkg_path(&idris_src_dir)),
            ("builddir", &quote_ipkg_path(&idris_build_dir)),
            ("outputdir", &quote_ipkg_path(&idris_output_dir)),
        ],
    );
    fs::write(&generated_pkg, rendered_package).expect("write generated Idris package");

    let generated_c = idris_output_dir.join(format!("{executable_name}.c"));

    let idris_bin = env_tool("IDRIS2", "idris2");
    let clang_bin = resolve_clang();
    let objcopy_bin = env_tool("LLVM_OBJCOPY", "llvm-objcopy");
    let ar_bin = env_tool("LLVM_AR", "llvm-ar");

    run(
        Command::new(&idris_bin)
            .arg("--build")
            .arg(&generated_pkg)
            .arg("--cg")
            .arg("refc")
            .env("IDRIS2_CC", "true"),
        "generate refc C from Idris2 package",
    );

    let mut clang = Command::new(&clang_bin);
    clang
        .arg("-target")
        .arg(clang_target(&target))
        .arg("-ffreestanding")
        .arg("-fno-stack-protector")
        .arg("-fno-builtin")
        .arg("-nostdlib")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-I")
        .arg(&include_dir)
        .arg("-c")
        .arg(&generated_c)
        .arg("-o")
        .arg(&generated_obj);

    if target == "riscv64gc-unknown-none-elf" {
        clang.arg("-march=rv64gc").arg("-mabi=lp64d");
    }

    run(&mut clang, "cross-compile generated Idris2 C");

    run(
        Command::new(&objcopy_bin)
            .arg("--redefine-sym")
            .arg("main=idris2_generated_main")
            .arg(&generated_obj),
        "rename refc standalone main symbol",
    );

    run(
        Command::new(&ar_bin)
            .arg("crs")
            .arg(&generated_archive)
            .arg(&generated_obj),
        "archive generated Idris2 object",
    );

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=kernel_idris_entry");
}

fn emit_rerun_if_changed_tree(root: &Path, extensions: &[&str]) {
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("read directory {}: {err}", dir.display()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|err| panic!("list directory {}: {err}", dir.display()));
        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();

            if path.is_dir() {
                stack.push(path);
                continue;
            }

            if matches_extension(&path, extensions) {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

fn matches_extension(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| extensions.iter().any(|candidate| candidate == &ext))
}

fn extract_ipkg_field(template: &str, key: &str) -> Option<String> {
    template.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        if name.trim() == key {
            Some(value.trim().trim_matches('"').to_owned())
        } else {
            None
        }
    })
}

fn render_build_ipkg(package_manifest: &str, replacements: &[(&str, &str)]) -> String {
    let mut rendered = Vec::new();
    let mut replaced = vec![false; replacements.len()];

    for line in package_manifest.lines() {
        let replacement = line.split_once('=').and_then(|(name, _)| {
            let key = name.trim();
            replacements
                .iter()
                .enumerate()
                .find(|(_, (candidate, _))| *candidate == key)
                .map(|(index, (candidate, value))| {
                    replaced[index] = true;
                    format!("{candidate} = {value}")
                })
        });

        rendered.push(replacement.unwrap_or_else(|| line.to_owned()));
    }

    for ((key, _), was_replaced) in replacements.iter().zip(replaced.iter()) {
        assert!(
            *was_replaced,
            "Idris package manifest must declare {key} = ..."
        );
    }

    let mut rendered = rendered.join("\n");
    if package_manifest.ends_with('\n') {
        rendered.push('\n');
    }
    rendered
}

fn quote_ipkg_path(path: &Path) -> String {
    let rendered = path.display().to_string();
    format!(
        "\"{}\"",
        rendered.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

fn env_tool(var: &str, fallback: &str) -> OsString {
    env::var_os(var).unwrap_or_else(|| OsString::from(fallback))
}

fn resolve_clang() -> OsString {
    if let Some(explicit) = env::var_os("CLANG") {
        return explicit;
    }

    let output = Command::new("clang")
        .arg("-print-prog-name=clang")
        .output()
        .ok();

    if let Some(output) = output
        && output.status.success()
    {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if !path.is_empty() {
            return OsString::from(path);
        }
    }

    OsString::from("clang")
}

fn clang_target(target: &str) -> &'static str {
    match target {
        "riscv64gc-unknown-none-elf" => "riscv64-unknown-elf",
        "x86_64-unknown-linux-gnu" => "x86_64-unknown-linux-gnu",
        _ => panic!("unsupported target for kernel build: {target}"),
    }
}

fn run(cmd: &mut Command, what: &str) {
    let status = cmd.status().unwrap_or_else(|err| {
        panic!("{what} failed to start: {err}");
    });

    if !status.success() {
        panic!("{what} failed with status {status}");
    }
}
