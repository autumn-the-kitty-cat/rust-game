let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  shellPackages = with pkgs; [
    toolchain
    rust-analyzer
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
