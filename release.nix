# For running on Hydra
{
  adc2tcp = import ./default.nix {
    mozillaOverlay = import <mozillaOverlay>;
    rustRestrictedManifest = true;
  };
}
