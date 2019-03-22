# For running on Hydra
{ pkgs ? import <nixpkgs> {},
  rustManifest ? ./nix/channel-rust-nightly.toml
}:

with pkgs;
let
  gdb = callPackage ./nix/gdb.nix {};
  adc2tcp = callPackage ./default.nix {
    inherit rustManifest;
    mozillaOverlay = import <mozillaOverlay>;
  };
in
{
  build = lib.hydraJob adc2tcp;
  gdb = lib.hydraJob gdb;
}
