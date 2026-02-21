{
  description = "Simple (-ish) Ground Penetrating Radar software";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          radal = import ./default.nix { inherit pkgs; };

        in
        {
          devShells.default = import ./shell.nix { inherit pkgs; };
          defaultPackage = radal;
          packages = {
            inherit radal;
            default = radal;
          };
        }

      ) // {
      overlays.default = final: prev: {
        radal = import ./default.nix { pkgs = final; };
      };
    };
}
