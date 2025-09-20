{
  pkgs ? import <nixpkgs> { },
}:
let
  libs =
    with pkgs;
    [
      udev
      alsa-lib
      alsa-lib-with-plugins
      vulkan-loader
      libxkbcommon
      wayland
      rustc
      cargo
      rustfmt
      rust-analyzer
      clippy
    ]
    ++ (with xorg; [
      libX11
      libXcursor
      libXi
      libXrandr
    ]);
in
with pkgs;
mkShell {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = libs;
  LD_LIBRARY_PATH = lib.makeLibraryPath libs;
  RUST_BACKTRACE = 1;
}
