{ stdenv, callPackage }:

let
  adc2tcp = callPackage (import ./adc2tcp.nix) {};
in
stdenv.mkDerivation {
  name = "adc2tcp";
  buildInputs = [
    adc2tcp
  ];
  src = ./.;
  noBuild = true;
  installPhase = ''
    find ${adc2tcp} -type f
  '';
}
