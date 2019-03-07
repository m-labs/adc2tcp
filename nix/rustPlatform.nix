{ recurseIntoAttrs, makeRustPlatform, rustChannelOfTargets, rustChannel ? "nightly" }:

let
  targets = [
    # "x86_64-unknown-linux-gnu"
    # "thumbv6m-none-eabi"
    # "thumbv7em-none-eabi"
    "thumbv7em-none-eabihf"
  ];
  rust = builtins.trace "rustChannel: selected channel ${rustChannel}"
    rustChannelOfTargets rustChannel null targets;
in
makeRustPlatform {
  rustc = rust;
  cargo = rust;
}
