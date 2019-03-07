{ stdenv, openocd }:

stdenv.mkDerivation {
  name = "openocd-nucleo-f429zi";
  buildInputs = [
    openocd
  ];
  src = ./.;
  noBuild = true;
  installPhase = ''
    mkdir -p $out/bin
    cat >> $out/bin/openocd-nucleo-f429zi <<EOF
    #!/usr/bin/env bash
    ${openocd}/bin/openocd \
      -f ${openocd}/share/openocd/scripts/interface/stlink-v2-1.cfg \
      -f ${openocd}/share/openocd/scripts/target/stm32f4x.cfg \
      -c "init" \
      "\$@"
    EOF
    chmod +x $out/bin/openocd-nucleo-f429zi
  '';
}
