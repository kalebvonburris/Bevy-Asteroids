# Windows cross-compilation environment for Bevy
{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" ];
  };
  
  # MinGW cross-compilation packages
  mingwPkgs = pkgs.pkgsCross.mingwW64;

in pkgs.mkShell {
  name = "bevy-windows-cross";
  
  buildInputs = [
    rustToolchain
    pkgs.pkg-config
    mingwPkgs.buildPackages.gcc
    mingwPkgs.windows.mingw_w64
    mingwPkgs.windows.mcfgthreads
  ];

  shellHook = ''
    # Configure Rust for cross-compilation
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
    export CC_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
    export CXX_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-g++"
    export AR_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-ar"
    
    # Set up library paths for MinGW (using mcfgthreads instead of pthreads)
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_PATH="${mingwPkgs.windows.mingw_w64}/lib/pkgconfig:${mingwPkgs.windows.mcfgthreads}/lib/pkgconfig"
    
    # Ensure the cross-compilation sysroot is available
    export BINDGEN_EXTRA_CLANG_ARGS="-I${mingwPkgs.windows.mingw_w64}/include -I${mingwPkgs.windows.mcfgthreads}/include"
    
    # Use static linking and mcfgthreads
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-C target-feature=+crt-static -L ${mingwPkgs.windows.mingw_w64}/lib -L ${mingwPkgs.windows.mcfgthreads}/lib"
    
    # Add the MinGW bin directory to PATH for runtime tools
    export PATH="${mingwPkgs.buildPackages.gcc}/bin:$PATH"
    
    echo "Windows cross-compilation environment"
    echo "Build with: cargo build --target=x86_64-pc-windows-gnu --release"
  '';
}