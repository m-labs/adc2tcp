# For running on Hydra
{ pkgs ? import <nixpkgs> {}}:

{
  adc2tcp = pkgs.lib.hydraJob (import ./default.nix {
    mozillaOverlay = import <mozillaOverlay>;
    rustRestrictedManifest = true;
  });
}
