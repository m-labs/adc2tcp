# TODO: gdb 8.2.1 from NixOS >= 19.XX is multiarch by default.
# remove the following as `gdb` is already in scope

{ stdenv, gdb }:

gdb.override {
  stdenv = stdenv.override {
    targetPlatform = {
      config = "arm-none-eabihf";
      libc = "newlib";
    };
  };
}
