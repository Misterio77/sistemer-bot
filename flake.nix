{
  description = "Bot do BSI 020 no telegram";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      name = "sistemer-bot";
      overlay = final: prev: {
        ${name} = final.callPackage ./default.nix { };
      };
      overlays = [ overlay ];
    in
    rec {
      inherit overlay overlays;

      nixosModules."${name}" = import ./module.nix;
      nixosModule = nixosModules."${name}";
    } //
    (utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs { inherit system overlays; });
      in
      rec {
        # nix build
        packages.${name} = pkgs.${name};
        defaultPackage = packages.${name};

        # nix run
        apps.${name} = utils.lib.mkApp { drv = packages.${name}; };
        defaultApp = apps.${name};

        # nix develop
        devShell = pkgs.mkShell {
          inputsFrom = [ defaultPackage ];
          buildInputs = with pkgs;
            [
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
        };
      }
    ));
}
