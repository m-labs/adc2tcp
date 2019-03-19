# For running on Hydra

import ./default.nix {
  mozillaOverlay = import <mozillaOverlay>;
  rustRestrictedManifest = true;
}
