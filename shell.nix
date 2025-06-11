{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "pompom";
  packages = with pkgs; [ pkg-config alsa-lib pipewire ];
}
