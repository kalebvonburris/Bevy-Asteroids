# Windows cross-compilation environment for Bevy
{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Use stable Rust for better compatibility with cross-compilation
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    targets = [ 
      "x86_64-unknown-linux-gnu" 
      "x86_64-pc-windows-gnu" 
      "wasm32-unknown-unknown" 
    ];
    extensions = [ "rust-src" ];
  };
  
  # MinGW cross-compilation packages
  mingwPkgs = pkgs.pkgsCross.mingwW64;

in pkgs.mkShell {
  name = "bevy-windows-cross";
  
  buildInputs = [
    rustToolchain
    pkgs.pkg-config
    
    # Cross-compilation toolchain
    mingwPkgs.buildPackages.gcc
    mingwPkgs.windows.mingw_w64
    mingwPkgs.windows.mcfgthreads
    
    # Additional libraries that might be needed
    mingwPkgs.windows.pthreads
  ];

  shellHook = ''
    # Configure Rust cross-compilation toolchain
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
    export CC_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
    export CXX_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-g++"
    export AR_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-ar"
    
    # Configure pkg-config for cross-compilation
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_PATH="${mingwPkgs.windows.mingw_w64}/lib/pkgconfig:${mingwPkgs.windows.mcfgthreads}/lib/pkgconfig:${mingwPkgs.windows.pthreads}/lib/pkgconfig"
    
    # Set up include paths for bindgen
    export BINDGEN_EXTRA_CLANG_ARGS="-I${mingwPkgs.windows.mingw_w64}/include -I${mingwPkgs.windows.mcfgthreads}/include -I${mingwPkgs.windows.pthreads}/include"
    
    # Configure library paths and linking flags
    # Removed crt-static to avoid threading library conflicts
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L ${mingwPkgs.windows.mingw_w64}/lib -L ${mingwPkgs.windows.mcfgthreads}/lib -L ${mingwPkgs.windows.pthreads}/lib"
    
    # Add MinGW tools to PATH
    export PATH="${mingwPkgs.buildPackages.gcc}/bin:$PATH"
    
    echo "✓ Windows cross-compilation environment configured"
    echo "  Rust toolchain: $(rustc --version)"
    echo "  Target: x86_64-pc-windows-gnu"
    echo ""
    echo "Build commands:"
    echo "  cargo build --target=x86_64-pc-windows-gnu --release"
    echo "  cargo run --target=x86_64-pc-windows-gnu"
  '';
}