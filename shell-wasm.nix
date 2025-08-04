# WASM development environment for Bevy (fixed for existing Rust installations)
{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" "x86_64-unknown-linux-gnu" ];
    extensions = [ "rust-src" ];
  };

in pkgs.mkShell {
  name = "bevy-wasm-dev";
  
  buildInputs = [
    rustToolchain
    pkgs.pkg-config
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.wasm-pack
    pkgs.wasm-bindgen-cli
    pkgs.binaryen
    pkgs.llvm
    pkgs.nodejs
    pkgs.git
    pkgs.glibc
    pkgs.glibc.dev
    pkgs.gcc
    pkgs.binutils
  ];

  shellHook = ''
    # Force isolation from existing Rust installations
    unset RUSTUP_HOME
    unset RUSTUP_TOOLCHAIN
    unset RUSTC_WRAPPER
    unset RUSTC
    unset RUST_SRC_PATH
    
    # Set up completely isolated cargo environment
    export CARGO_HOME="$PWD/.nix-cargo"
    mkdir -p "$CARGO_HOME"
    
    # Force Rust to use only Nix-provided sysroot and libraries
    export RUSTFLAGS="--sysroot ${rustToolchain} -L ${rustToolchain}/lib/rustlib/x86_64-unknown-linux-gnu/lib"
    
    # Configure OpenSSL paths explicitly
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
    
    # Override library search paths to prevent finding old rustup libs
    export LD_LIBRARY_PATH="${pkgs.glibc}/lib:${pkgs.gcc.cc.lib}/lib:${pkgs.openssl.out}/lib"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    
    # Use a temporary directory for cargo target to avoid impure path issues
    export CARGO_TARGET_DIR="/tmp/cargo-target-$(whoami)"
    mkdir -p "$CARGO_TARGET_DIR"

    # Add Cargo bin directory to PATH
    export PATH="$CARGO_HOME/bin:$PATH"

    # Set explicit Rust paths to prevent cargo from searching elsewhere
    export RUST_SYSROOT="${rustToolchain}"
    export RUSTLIB="${rustToolchain}/lib/rustlib"
    
    # Install bevy CLI if not already present
    if ! command -v bevy &> /dev/null; then
      echo "Installing bevy CLI..."
      cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-v0.1.0-alpha.1 --locked bevy_cli
    fi
    
    echo "WASM development environment"
    echo "Build with: bevy build --release web --bundle
  '';
}
