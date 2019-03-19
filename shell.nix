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
  rustPlatform = callPackage (import ./nix/rustPlatform.nix) {};
  openocd = callPackage (import ./nix/openocd.nix) {};
  # TODO: gdb 8.2.1 from NixOS >= 19.XX is multiarch by default.
  # remove the following as `gdb` is already in scope
  gdb = pkgs.gdb.override {
    stdenv = stdenv.override {
      targetPlatform = {
        config = "arm-none-eabihf";
        libc = "newlib";
      };
    };
  };
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
