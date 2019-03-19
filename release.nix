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
  build = runCommand "build-adc2tcp" {
    buildInputs = [ adc2tcp ];
  };
}
