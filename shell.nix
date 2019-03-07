{ rustChannel ? "nightly" }:

let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
with import <nixpkgs> { overlays = [ mozillaOverlay ]; };
let
  rustPlatform = recurseIntoAttrs (
    callPackage (import ./nix/rustPlatform.nix) { inherit rustChannel; }
  );
in
# mkShell {
#   inputsFrom = with rustPlatform.rust; [
#     rustc cargo
#   ];
# }
stdenv.mkDerivation {
  name = "adc2tcp-env";
  buildInputs = with rustPlatform.rust; [
    rustc cargo
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
