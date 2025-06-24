let
  pkgs = import <nixpkgs> {};
  shellPackages = with pkgs; [
    rust-analyzer
    rustfmt
    cargo
    clippy
    pkg-config
    udev
    alsa-lib-with-plugins
    vulkan-loader
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
  ];

  libPath = with pkgs; lib.makeLibraryPath shellPackages;
in
pkgs.mkShell {
  packages = shellPackages;
  LD_LIBRARY_PATH = libPath; 
  WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1;
}
