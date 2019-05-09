{ stdenv, rustPlatform, cacert, git, cargo-vendor }:

with rustPlatform;
let
  sha256 = "0071fn2gj976s20nv6wfjyi0ddcsq17sbpdxkfl0r5hwia5gixph";
  fetchcargo = import ./fetchcargo.nix {
    inherit stdenv cacert git cargo-vendor;
    inherit (rust) cargo;
  };
  adc2tcpDeps = fetchcargo {
    name = "adc2tcp";
    src = ../.;
    inherit sha256;
  };
in

buildRustPackage rec {
  name = "adc2tcp";
  version = "0.0.0";

  src = ../.;
  cargoSha256 = sha256;

  buildInputs = [ adc2tcpDeps ];
  patchPhase = ''
    cat >> .cargo/config <<EOF
    [source.crates-io]
    replace-with = "vendored-sources"

    [source.vendored-sources]
    directory = "${adc2tcpDeps}"
    EOF
  '';

  buildPhase = ''
    export CARGO_HOME=$(mktemp -d cargo-home.XXX)
    cargo build --release
  '';

  doCheck = false;
  installPhase = ''
    mkdir -p $out/lib
    cp target/thumbv7em-none-eabihf/release/adc2tcp $out/lib/
  '';
}
