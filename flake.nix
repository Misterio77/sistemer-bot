{
  description = "Bot do BSI 020 no telegram";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib."${system}";
      in rec {
        packages.backend-uget = naersk-lib.buildPackage {
          pname = "sistemerb-bot";
          root = ./.;
        };
        defaultPackage = packages.sistemer-bot;

        apps.backend-uget = flake-utils.lib.mkApp {
          drv = packages.sistemer-bot;
        };
        defaultApp = apps.sistemer-bot;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
            ];
        };
      });
}
