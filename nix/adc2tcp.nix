{ stdenv, rustPlatform, cacert, git, cargo-vendor, coreutils }:

with rustPlatform;
let
  sha256 = "19cdc0lkm9247n6kf23ki66gysz530j1x2lfnzq7n0cpcs53q3h3";
  fetchcargo = import ./fetchcargo.nix {
    inherit stdenv cacert git cargo-vendor coreutils;
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
