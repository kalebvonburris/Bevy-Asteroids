# Default Linux development environment for Bevy
{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "x86_64-unknown-linux-gnu" ];
    extensions = [ "rust-src" ];
  };
  
  systemLibs = with pkgs; [
    udev
    systemd
    vulkan-loader
    vulkan-tools
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.libXxf86vm
    libGL
    libxkbcommon
    wayland
    mesa
    alsa-lib
    lld
  ];

in pkgs.mkShell {
  name = "bevy-linux-dev";
  
  buildInputs = [
    rustToolchain
    pkgs.pkg-config
  ] ++ systemLibs;

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath systemLibs}:$LD_LIBRARY_PATH
    export PKG_CONFIG_PATH="${pkgs.systemd.dev}/lib/pkgconfig:${pkgs.udev.dev}/lib/pkgconfig:${pkgs.alsa-lib.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
    export PATH="$HOME/.cargo/bin:$PATH"

    echo "Native Linux development environment"
    echo "Build with: cargo build"
  '';
}
