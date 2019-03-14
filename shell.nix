{ rustChannel ? "nightly" }:

let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
with import <nixpkgs> { overlays = [ mozillaOverlay ]; };
let
  rustPlatform = callPackage (import ./nix/rustPlatform.nix) { inherit rustChannel; };
  openocd = callPackage (import ./nix/openocd.nix) {};
in
stdenv.mkDerivation {
  name = "adc2tcp-env";
  buildInputs = with rustPlatform.rust; [
    rustc cargo
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "Starting openocd…"
    ${openocd}/bin/openocd-nucleo-f429zi &

    # Let openocd output scroll by
    sleep 1

    echo "Run 'cargo build --release --features=semihosting'"
    echo "Then 'gdb target/thumbv7em-none-eabihf/release/adc2tcp'"
  '';
}
