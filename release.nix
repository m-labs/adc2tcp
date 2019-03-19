# For running on Hydra
{ pkgs ? import <nixpkgs> {},
  rustManifest ? ./nix/channel-rust-nightly.toml
}:

{
  adc2tcp = pkgs.lib.hydraJob (import ./default.nix {
    inherit rustManifest;
    mozillaOverlay = import <mozillaOverlay>;
  });
}
