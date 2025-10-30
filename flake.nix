{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk-lib = pkgs.callPackage naersk { };

        ansi-square = naersk-lib.buildPackage {
          pname = "ansi-square";
          src = ./.;
        };
      in
      {
        packages = {
          inherit ansi-square;
          default = ansi-square;
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
          ];

          packages = with pkgs; [
            rust-analyzer
          ];
        };
      }
    );
}
