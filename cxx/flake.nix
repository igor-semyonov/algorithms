{
  description = "Flake for cxx algorithms";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    devShells.${system}.default = pkgs.mkShell rec {
      nativeBuildInputs = with pkgs; [
      ];
      buildInputs = with pkgs; [
          gcc
          cmake
          clang-tools
          gnumake
          gdb
      ];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    };
  };
}
