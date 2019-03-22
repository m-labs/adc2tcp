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
    sha256 = "0071fn2gj976s20nv6wfjyi0ddcsq17sbpdxkfl0r5hwia5gixph";
  };
in

buildRustPackage rec {
  name = "adc2tcp";
  version = "0.0.0";

  src = ../.;
  cargoSha256 = "0071fn2gj976s20nv6wfjyi0ddcsq17sbpdxkfl0r5hwia5gixph";

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
    mkdir -p $out/lib $out/nix-support
    cp target/thumbv7em-none-eabihf/release/adc2tcp $out/lib/
    echo file binary-dist $out/lib/adc2tcp >> $out/nix-support/hydra-build-products
  '';
}
