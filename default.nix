{ # Use master branch of the overlay by default
  mozillaOverlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz),
  rustManifest ? builtins.fetchurl "https://static.rust-lang.org/dist/channel-rust-nightly.toml"
}:

let
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
in
with pkgs;
let
  rustPlatform = recurseIntoAttrs (callPackage ./nix/rustPlatform.nix {
    inherit rustManifest;
  });
  adc2tcp = callPackage ./nix/adc2tcp.nix { inherit rustPlatform; };
  openocd = callPackage ./nix/openocd.nix {};
in
stdenv.mkDerivation {
  name = "adc2tcp-dist";
  buildInputs = [
    adc2tcp
    openocd
    makeWrapper
  ];
  src = ./.;
  dontBuild = true;

  installPhase =
    let
      firmwareBinary = "$out/lib/adc2tcp.elf";
      openOcdFlags = [
        "-c" "reset halt"
        "-c" "flash write_image erase ${firmwareBinary}"
        "-c" "verify_image ${firmwareBinary}"
        "-c" "reset run"
        "-c" "shutdown"
      ];
    in ''
      mkdir -p $out/bin $out/lib $out/nix-support

      ln -s ${adc2tcp}/lib/adc2tcp ${firmwareBinary}

      makeWrapper ${openocd}/bin/openocd-nucleo-f429zi $out/bin/flash-adc2tcp \
        --add-flags "${lib.escapeShellArgs openOcdFlags}"

      echo file binary-dist ${firmwareBinary} >> $out/nix-support/hydra-build-products
    '';
}
