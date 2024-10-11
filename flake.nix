{
  description = "Rust Flake Template";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath = with pkgs; lib.makeLibraryPath [
        ];
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            libGL
            libGLU
            glew
            cmake
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXfixes
            libxkbcommon
            fontconfig
            pango
            cairo
            dbus
            wayland
            pkg-config
            rust-analyzer
            rustfmt
            (rust-bin.nightly.latest.default.override {
              extensions = ["rust-analyzer"];
            })
          ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
