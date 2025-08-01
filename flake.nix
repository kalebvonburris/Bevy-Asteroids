{
  description = "A flake for building a Bevy (Rust) application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "wasm32-unknown-unknown" ];
        };
        
        # MinGW cross-compilation packages
        mingwPkgs = pkgs.pkgsCross.mingwW64;
      in
      {
        devShells = {
          # Default shell with native Linux development
          default = pkgs.mkShell {
            buildInputs = [
              rustToolchain
              pkgs.pkg-config
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.vulkan-tools
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
              pkgs.xorg.libXxf86vm
              pkgs.libGL
              pkgs.libxkbcommon
              pkgs.wayland
              pkgs.mesa
              pkgs.alsa-lib
              pkgs.lld
            ];
            shellHook = ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
                pkgs.vulkan-loader
                pkgs.xorg.libX11
                pkgs.xorg.libXcursor
                pkgs.xorg.libXi
                pkgs.xorg.libXrandr
                pkgs.xorg.libXxf86vm
                pkgs.libGL
                pkgs.libxkbcommon
                pkgs.wayland
                pkgs.mesa
                pkgs.alsa-lib
                pkgs.lld
              ]}:$LD_LIBRARY_PATH
              
              # Add Cargo bin directory to PATH
              export PATH="$HOME/.cargo/bin:$PATH"
              
              echo "Native Linux development environment"
              echo "Build with: cargo build"
            '';
          };
          
          # Windows cross-compilation shell
          windows = pkgs.mkShell {
            buildInputs = [
              rustToolchain
              pkgs.pkg-config
              
              # Use the cross-compilation toolchain properly
              mingwPkgs.buildPackages.gcc
              mingwPkgs.windows.mingw_w64
              # Override mcfgthreads to include static libraries
              (mingwPkgs.windows.mcfgthreads.overrideAttrs {
                dontDisableStatic = true;
              })
            ];
            
            # Set up proper environment variables for cross-compilation
            shellHook = ''
              # Configure Rust for cross-compilation
              export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
              export CC_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-gcc"
              export CXX_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-g++"
              export AR_x86_64_pc_windows_gnu="${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32-ar"
              
              # Set up library paths for MinGW
              export PKG_CONFIG_ALLOW_CROSS=1
              export PKG_CONFIG_PATH="${mingwPkgs.windows.mingw_w64}/lib/pkgconfig:${mingwPkgs.windows.mcfgthreads}/lib/pkgconfig"
              
              # Ensure the cross-compilation sysroot is available
              export BINDGEN_EXTRA_CLANG_ARGS="-I${mingwPkgs.windows.mingw_w64}/include -I${mingwPkgs.windows.mcfgthreads}/include"
              
              # Add the MinGW bin directory to PATH for runtime tools
              export PATH="${mingwPkgs.buildPackages.gcc}/bin:$PATH"
              
              echo "Windows cross-compilation environment"
              echo "Build with: cargo build --target=x86_64-pc-windows-gnu --release"
            '';
          };
          
          wasm = pkgs.mkShell {
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
              # Clean environment for WASM
              unset CC
              unset CXX
              unset AR
              unset RANLIB
              unset STRIP
              
              # Explicitly set WASM-friendly tools
              export CC_wasm32_unknown_unknown="clang"
              export AR_wasm32_unknown_unknown="llvm-ar"
              
              # Install bevy cli
              cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-v0.1.0-alpha.1 --locked bevy_cli

              # Add Cargo bin directory to PATH
              export PATH="$HOME/.cargo/bin:$PATH"
              
              echo "WASM development environment"
              echo "Build with: bevy build --release web --bundle"
            '';
          };
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "bevy-asteroids";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = [
            pkgs.udev
            pkgs.vulkan-loader
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXi
            pkgs.xorg.libXrandr
            pkgs.xorg.libXxf86vm
            pkgs.libGL
            pkgs.libxkbcommon
            pkgs.wayland
            pkgs.mesa
          ];
        };
      }
    );
}