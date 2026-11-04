{ pkgs ? import <nixpkgs> { } }:
let
  runtimeLibraries = with pkgs; [
    wayland
    libxkbcommon
    vulkan-loader
    libGL
    libx11
    libxcursor
    libxi
    libxrandr
  ];
in
pkgs.mkShell {
  packages = with pkgs; [ pkg-config cargo rustc rustfmt clippy ];
  buildInputs = runtimeLibraries;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibraries;
}
