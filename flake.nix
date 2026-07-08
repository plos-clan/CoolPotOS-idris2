{
  description = "CPOS Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        idris2Pkg = pkgs.idris2;
        idris2Api = pkgs.idris2Packages.idris2Api;
        idris2Lsp = pkgs.idris2Packages.idris2Lsp;

        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "llvm-tools" ];
          targets = [ "riscv64gc-unknown-none-elf" ];
        };

        clang = pkgs.clang;
        lld = pkgs.lld;
        llvm = pkgs.llvm;

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            idris2Pkg
            idris2Api
            idris2Lsp
            rustToolchain
            clang
            lld
            llvm
            qemu
            gmp
            chez
            gdb
            lldb
            cargo-watch
            bacon
          ];

          CARGO_BUILD_TARGET = "riscv64gc-unknown-none-elf";
          CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_LINKER = "${lld}/bin/ld.lld";
          CC_riscv64gc_unknown_none_elf = "${clang}/bin/clang --target=riscv64-unknown-elf -nostdlib -ffreestanding";
          CXX_riscv64gc_unknown_none_elf = "${clang}/bin/clang++ --target=riscv64-unknown-elf -nostdlib -ffreestanding";

        };
      }
    );
}
