{ # Use master branch of the overlay by default
  mozillaOverlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz),
  pkgs ? import <nixpkgs> { overlays = [ mozillaOverlay ]; },
  rustManifest ? builtins.fetchurl "https://static.rust-lang.org/dist/channel-rust-nightly.toml"
}:

with pkgs;
let
  rustPlatform = recurseIntoAttrs (callPackage (import ./nix/rustPlatform.nix) {
    inherit rustManifest;
  });
  adc2tcp = callPackage (import ./nix/adc2tcp.nix) { inherit rustPlatform; };
  openocd = callPackage (import ./nix/openocd.nix) {};
in
stdenv.mkDerivation {
  name = "adc2tcp-dist";
  buildInputs = [
    adc2tcp
    openocd
  ];
  src = ./.;
  dontBuild = true;

  installPhase = ''
    mkdir -p $out/bin $out/lib

    BIN=$out/lib/adc2tcp
    ln -s ${adc2tcp}/lib/adc2tcp $BIN
    cat >> $out/bin/flash-adc2tcp <<EOF
    #!/usr/bin/env bash
    ${openocd}/bin/openocd-nucleo-f429zi \
      -c "reset halt" \
      -c "flash write_image erase $BIN" \
      -c "verify_image $BIN" \
      -c "reset run" \
      -c "shutdown"
    EOF
    chmod +x $out/bin/flash-adc2tcp
  '';
}
