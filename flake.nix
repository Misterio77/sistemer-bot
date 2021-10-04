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
        name = "sistemer-bot";
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib."${system}";
      in rec {
        packages.${name} = naersk-lib.buildPackage {
          pname = name;
          buildInputs = with pkgs; [ openssl pkg-config ];
          root = ./.;
        };
        defaultPackage = packages.${name};

        apps.${name} = flake-utils.lib.mkApp {
          drv = packages.${name};
        };
        defaultApp = apps.${name};

        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
              pkgconfig
              openssl
            ];
        };
      });
}
