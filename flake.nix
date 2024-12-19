{
  description = "A chip8 emu implemented in rust with the bevy game engine";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";

      overlays = [ (import rust-overlay) ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.${system}.default =
        pkgs.mkShell.override
          {
            stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
          }
          {
            packages = with pkgs; [
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
              })
              taplo
              pandoc
            ];
          };
    };
}
