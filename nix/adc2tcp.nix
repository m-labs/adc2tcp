{ stdenv, rustPlatform }:

with rustPlatform;

buildRustPackage rec {
  name = "adc2tcp";
  version = "0.0.0";

  src = ../.;
  cargoSha256 = "0q3cn3jzgmrqiymisxymn19vbnnpsj7if052w5zh25x9ikin6lpl";

  doCheck = false;
  installPhase = ''
    mkdir -p $out/lib
    cp target/thumbv7em-none-eabihf/release/adc2tcp $out/lib/
  '';
}
