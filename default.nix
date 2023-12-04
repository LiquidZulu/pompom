with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "pompom";
  buildInputs = [ pkg-config alsa-lib libudev-zero libinput ]
    ++ (with xorg; [ libX11 libXtst ]);
}
