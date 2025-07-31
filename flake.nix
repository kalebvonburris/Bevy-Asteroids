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
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        devShells.default = pkgs.mkShell {
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
            ]}:$LD_LIBRARY_PATH
          '';
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