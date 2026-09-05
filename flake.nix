{
  description = "Bevy Asteroids - Linux, Windows (cross) and WASM development/release environments";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    # Linux only: the native shell needs udev/wayland/X11, and the Windows and
    # WASM shells cross-compile from a Linux host.
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        # e.g. "x86_64-unknown-linux-gnu"; derived so this also works on aarch64.
        hostTarget = pkgs.stdenv.hostPlatform.rust.rustcTarget;

        # Every shell ships rust-analyzer alongside rust-src, so an editor LSP
        # client (nvim) gets an analyzer built against the same toolchain as the
        # shell, and can jump into core/alloc/std sources.
        rustExtensions = [ "rust-src" "rust-analyzer" ];

        mkStableToolchain = targets:
          pkgs.rust-bin.stable.latest.default.override {
            inherit targets;
            extensions = rustExtensions;
          };

        # `nightly.latest` is whatever nightly was published today, and
        # rust-analyzer is occasionally missing from one of those. This walks
        # back to the newest nightly that actually has every component asked for.
        mkNightlyToolchain = targets:
          pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              inherit targets;
              extensions = rustExtensions;
            });

        linuxToolchain = mkStableToolchain [ hostTarget ];
        windowsToolchain = mkStableToolchain [ hostTarget "x86_64-pc-windows-gnu" ];
        wasmToolchain = mkNightlyToolchain [ "wasm32-unknown-unknown" hostTarget ];

        # Where the rust-src component lands; rust-analyzer reads this.
        rustSrcPath = toolchain: "${toolchain}/lib/rustlib/src/rust/library";

        # Libraries Bevy links against or dlopen()s at runtime on Linux.
        systemLibs = with pkgs; [
          udev # systemd-minimal-libs, provides libudev
          systemd
          vulkan-loader
          vulkan-tools
          libx11
          libxcursor
          libxi
          libxrandr
          libxxf86vm
          libGL
          libxkbcommon
          wayland
          mesa
          alsa-lib
          lld
        ];

        # MinGW cross-compilation packages for the Windows target.
        mingwPkgs = pkgs.pkgsCross.mingwW64;
        mingwCC = "${mingwPkgs.buildPackages.gcc}/bin/x86_64-w64-mingw32";

        # wasm-bindgen refuses to run unless the CLI and the crate agree exactly on
        # the bindgen schema version, so surface a drift after `nix flake update`
        # here rather than after a ten-minute wasm build.
        lockedWasmBindgen =
          (lib.findFirst (p: p.name == "wasm-bindgen") null
            (builtins.fromTOML (builtins.readFile ./Cargo.lock)).package).version;
        wasm-bindgen-cli = lib.warnIf
          (pkgs.wasm-bindgen-cli.version != lockedWasmBindgen)
          ''
            wasm-bindgen version mismatch: nixpkgs ships the ${pkgs.wasm-bindgen-cli.version} CLI
            but Cargo.lock pins the ${lockedWasmBindgen} crate. `wasm-bindgen` will refuse to run.
            Fix by running `cargo update -p wasm-bindgen --precise ${pkgs.wasm-bindgen-cli.version}`,
            or by pinning the CLI in flake.nix with `pkgs.buildWasmBindgenCli`.
          ''
          pkgs.wasm-bindgen-cli;
      in
      {
        devShells = rec {
          default = linux;

          # Native Linux development.
          linux = pkgs.mkShell {
            name = "bevy-linux-dev";

            buildInputs = [
              linuxToolchain
              pkgs.pkg-config
            ] ++ systemLibs;

            shellHook = ''
              export LD_LIBRARY_PATH="${lib.makeLibraryPath systemLibs}:$LD_LIBRARY_PATH"
              export PKG_CONFIG_PATH="${lib.makeSearchPathOutput "dev" "lib/pkgconfig" (with pkgs; [ systemd udev alsa-lib ])}:$PKG_CONFIG_PATH"
              export PATH="$HOME/.cargo/bin:$PATH"
              export RUST_SRC_PATH="${rustSrcPath linuxToolchain}"

              echo "Native Linux development environment"
              echo "  rust-analyzer: $(command -v rust-analyzer)"
              echo "Build with: cargo build"
            '';
          };

          # Windows cross-compilation (x86_64-pc-windows-gnu).
          windows = pkgs.mkShell {
            name = "bevy-windows-cross";

            buildInputs = [
              windowsToolchain
              pkgs.pkg-config

              # Cross-compilation toolchain
              mingwPkgs.buildPackages.gcc
              mingwPkgs.windows.mingw_w64
              mingwPkgs.windows.mcfgthreads
              mingwPkgs.windows.pthreads
            ];

            shellHook = ''
              # Point Rust and the `cc` crate at the MinGW toolchain.
              export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${mingwCC}-gcc"
              export CC_x86_64_pc_windows_gnu="${mingwCC}-gcc"
              export CXX_x86_64_pc_windows_gnu="${mingwCC}-g++"
              export AR_x86_64_pc_windows_gnu="${mingwCC}-ar"

              export PKG_CONFIG_ALLOW_CROSS=1
              export PKG_CONFIG_PATH="${lib.makeSearchPath "lib/pkgconfig" (with mingwPkgs.windows; [ mingw_w64 mcfgthreads pthreads ])}"
              export BINDGEN_EXTRA_CLANG_ARGS="${lib.concatMapStringsSep " " (p: "-I${p}/include") (with mingwPkgs.windows; [ mingw_w64 mcfgthreads pthreads ])}"

              # No crt-static here: it conflicts with the threading libraries.
              export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="${lib.concatMapStringsSep " " (p: "-L ${p}/lib") (with mingwPkgs.windows; [ mingw_w64 mcfgthreads pthreads ])}"

              export RUST_SRC_PATH="${rustSrcPath windowsToolchain}"

              echo "Windows cross-compilation environment"
              echo "  Rust toolchain: $(rustc --version)"
              echo "  Target: x86_64-pc-windows-gnu"
              echo ""
              echo "Build with: cargo build --target=x86_64-pc-windows-gnu --release"
            '';
          };

          # WASM / web builds.
          # Nightly, because .cargo/config.toml uses `-Z build-std` for wasm32.
          wasm = pkgs.mkShell {
            name = "bevy-wasm-dev";

            buildInputs = [
              wasmToolchain
              wasm-bindgen-cli # version-checked against Cargo.lock above
              pkgs.binaryen
              pkgs.nodejs
            ];

            shellHook = ''
              # Ignore any rustup install on the host; this shell provides the toolchain.
              unset RUSTUP_HOME RUSTUP_TOOLCHAIN RUSTC_WRAPPER RUSTC RUST_SRC_PATH

              # ...but re-point rust-analyzer at this shell's nightly std sources.
              export RUST_SRC_PATH="${rustSrcPath wasmToolchain}"

              echo "WASM development environment"
              echo "Build with:"
              echo "  cargo build --profile web-release --no-default-features --target wasm32-unknown-unknown"
              echo "  wasm-bindgen --no-typescript --target web --out-dir dist/build --out-name asteroids \\"
              echo "    target/wasm32-unknown-unknown/web-release/asteroids.wasm"
              echo "  cp web/index.html dist/ && cp -r assets dist/assets"
            '';
          };
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
