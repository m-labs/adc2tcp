{ recurseIntoAttrs, stdenv, lib,
  makeRustPlatform, rustChannelOfTargets,
  fetchurl, patchelf,
  rustManifest ? ./channel-rust-nightly.toml
}:

let
  targets = [
    # "x86_64-unknown-linux-gnu"
    # "thumbv6m-none-eabi"
    # "thumbv7m-none-eabi"
    # "thumbv7em-none-eabi"
    "thumbv7em-none-eabihf"
  ];
  rustChannel =
    lib.rustLib.fromManifestFile rustManifest {
      inherit stdenv fetchurl patchelf;
    };
  rust =
    if restrictedManifest
    then
      rustChannel.rust.override { inherit targets; }
    else
      rustChannelOfTargets "nightly" null targets;
in
makeRustPlatform {
  rustc = rust;
  cargo = rust;
}
