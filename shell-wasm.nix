# WASM development environment for Bevy
{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" ];
  };

in pkgs.mkShell {
  name = "bevy-wasm-dev";
  
  buildInputs = [
    rustToolchain
    pkgs.pkg-config
    pkgs.openssl
    pkgs.wasm-pack
    pkgs.wasm-bindgen-cli
    pkgs.binaryen
    pkgs.llvm
    pkgs.nodejs
  ];

  shellHook = ''
    # Add Cargo bin directory to PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    
    # Install bevy CLI if not already present
    if ! command -v bevy &> /dev/null; then
      echo "Installing bevy CLI..."
      cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-v0.1.0-alpha.1 --locked bevy_cli
    fi
    
    echo "WASM development environment"
    echo "Build with: bevy build --release web --bundle"
  '';
}