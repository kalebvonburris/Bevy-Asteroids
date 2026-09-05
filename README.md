# Asteroids

Simple asteroids clone made in Bevy.

Find the web version here: <https://kalebvonburris.github.io/Bevy-Asteroids/>

Or download a release from here: <https://github.com/kalebvonburris/Bevy-Asteroids/releases/latest>

## Controls

### PC

On a keyboard, use the `w`, `a`, `s`, and `d` keys or the arrow keys to turn and move. Press space bar to shoot a bullet.

## Development and Compiling

In order to work on the code or compile it yourself, I recommend installing [Nix](https://nixos.org/download/)
with [flakes enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes_temporarily).
This way, you can call `nix develop .#PLATFORM` with the environment of your choosing to compile the game for that platform.

Platforms:

- Linux: `nix develop` (or `nix develop .#linux`)
- Windows: `nix develop .#windows`
- Web: `nix develop .#wasm`

Each shell prints the build command to use when you enter it:

- Linux: `cargo build --release --no-default-features`
- Windows: `cargo build --release --no-default-features --target x86_64-pc-windows-gnu`
- Web: `cargo build --profile web-release --no-default-features --target wasm32-unknown-unknown`,
  then `wasm-bindgen --out-dir dist --target web target/wasm32-unknown-unknown/web-release/asteroids.wasm`

Every shell also provides `rust-analyzer` and `rust-src`, matched to that shell's toolchain, so an editor
started from inside the shell picks up the LSP with no separate rustup install.

Nixpkgs and the Rust toolchains are pinned in `flake.lock`; run `nix flake update` to move them forward.

## Credits

This [template](https://github.com/TheBevyFlock/bevy_new_2d/) for the initial code.

Reign Pagaran - Main Song (Eternity)
