# For running on Hydra
{ pkgs ? import <nixpkgs> {},
  rustManifest ? ./nix/channel-rust-nightly.toml
}:

with pkgs;
let
  adc2tcp = callPackage (import ./default.nix) {
    inherit rustManifest;
    mozillaOverlay = import <mozillaOverlay>;
  };
in
{
  build = lib.hydraJob adc2tcp;
}
