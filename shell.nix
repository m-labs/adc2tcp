{ rustChannel ? "nightly" }:

let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  # `lib.systems.examples.armhf-embedded` from nixpkgs master
  # (TODO: use directly starting with NixOS 19.0X)
  targetPlatform = {
    config = "arm-none-eabihf";
    libc = "newlib";
  };
in
with pkgs;
let
  rustPlatform = callPackage ./nix/rustPlatform.nix {};
  openocd = callPackage ./nix/openocd.nix {};
  gdb = callPackage ./nix/gdb.nix {};
in
stdenv.mkDerivation {
  name = "adc2tcp-env";
  buildInputs = with rustPlatform.rust; [
    rustc cargo gdb
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "Starting openocd…"
    ${openocd}/bin/openocd-nucleo-f429zi &

    # Let openocd output scroll by
    sleep 1

    echo "Run 'cargo run --release --features=semihosting'"
  '';
}
