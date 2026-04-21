{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
      	let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        with pkgs;
	      with python314Packages;
        {
          devShells.default = mkShell {
            buildInputs = [
              (python314.withPackages (ps: with ps; [
                matplotlib
                numpy
                black
              ]))
              cargo
            ];
          };
        }
      );
}
