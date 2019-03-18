{ stdenv, rustPlatform, cacert, git, cargo-vendor }:

with rustPlatform;
let
  fetchcargo = import ./fetchcargo.nix {
    inherit stdenv cacert git cargo-vendor;
    inherit (rust) cargo;
  };
  adc2tcpDeps = fetchcargo {
    name = "adc2tcp-deps";
    src = ../.;
    sha256 = "1wddmsdsqwfzld43g12l2r60ndfwi8ihwm03zzcs9722sirw49fz";
  };
in

buildRustPackage rec {
  name = "adc2tcp";
  version = "0.0.0";

  src = ../.;
  cargoSha256 = "1wddmsdsqwfzld43g12l2r60ndfwi8ihwm03zzcs9722sirw49fz";

  buildInputs = [ adc2tcpDeps ];
  patchPhase = ''
    cat >> .cargo/config <<EOF
    [source.crates-io]
    replace-with = "vendored-sources"

    [source.vendored-sources]
    directory = "${adc2tcpDeps}"

    [source."https://github.com/stm32-rs/stm32f4xx-hal"]
    git = "https://github.com/stm32-rs/stm32f4xx-hal"
    branch = "master"
    replace-with = "vendored-sources"

    [source."https://github.com/stm32-rs/stm32-eth"]
    git = "https://github.com/stm32-rs/stm32-eth"
    branch = "master"
    replace-with = "vendored-sources"
    EOF
  '';

  doCheck = false;
  installPhase = ''
    mkdir -p $out/lib
    cp target/thumbv7em-none-eabihf/release/adc2tcp $out/lib/
  '';
}
