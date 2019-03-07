let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
with import <nixpkgs> { overlays = [ mozillaOverlay ]; };
let
  rustPlatform = recurseIntoAttrs (callPackage (import ./nix/rustPlatform.nix) {});
in
callPackage (import ./nix/adc2tcp.nix) { inherit rustPlatform; }
