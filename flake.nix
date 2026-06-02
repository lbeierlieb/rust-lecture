{
  description = "Memory-safe Programming: An Introduction to Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = crane.mkLib pkgs;

        buildCargoProject =
          src:
          let
            commonArgs = {
              src = craneLib.cleanCargoSource src;
              strictDeps = true;
            };
          in
          craneLib.buildPackage (
            commonArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            }
          );

        week1 = buildCargoProject ./exercises/week1/zombiesim;
      in
      {
        packages.week1 = week1;

        devShells.default = craneLib.devShell { };
      }
    );
}
