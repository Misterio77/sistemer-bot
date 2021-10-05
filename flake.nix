{
  description = "Bot do BSI 020 no telegram";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-compat.flake = false;
  };
  outputs = { self, nixpkgs, flake-utils, flake-compat }:
  flake-utils.lib.eachDefaultSystem (system: 
    let
      name = "sistemer-bot";
      pkgs = (import nixpkgs { inherit system; });
    in rec {
      packages.${name} = (import ./Cargo.nix { inherit pkgs; }).rootCrate.build;
      defaultPackage = packages.${name};
      apps.${name} = flake-utils.lib.mkApp { drv = packages.${name}; };
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
    }
  );
}
