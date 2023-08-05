with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "pompom";
  buildInputs = [ pkg-config alsa-lib ];
}
