# For running on Hydra
{ pkgs ? import <nixpkgs> {},
  rustManifest ? ./nix/channel-rust-nightly.toml
}:

{
channel = pkgs.releaseTools.channel {
  name = "adc2tcp";
  adc2tcp = pkgs.lib.hydraJob (import ./default.nix {
    inherit rustManifest;
    mozillaOverlay = import <mozillaOverlay>;
    rustRestrictedManifest = true;
  });
}
