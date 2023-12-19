{
  description = "A crate to parse norg files with pandoc";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

      in
      {
        # For `nix build` & `nix run`:
        packages.default = naersk'.buildPackage {
          src = ./.;
          buildInputs = [ pkgs.pandoc ];
          PANDOC_PATH = pkgs.lib.getExe pkgs.pandoc;
        };

        # For `nix develop` (optional, can be skipped):
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
          buildInputs = [ pkgs.pandoc ];
        };
      }
    );
}
