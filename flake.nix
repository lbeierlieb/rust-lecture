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

        slides = pkgs.stdenv.mkDerivation {
          pname = "rust-lecture-slides";
          version = "0.1.0";

          src = ./slides;

          nativeBuildInputs = [ pkgs.texlive.combined.scheme-full ];

          phases = [
            "unpackPhase"
            "buildPhase"
            "installPhase"
          ];

          buildPhase = ''
            latexmk -pdf main.tex
          '';

          installPhase = ''
            mkdir $out
            mv main.pdf $out/main.pdf
          '';
        };
        slides-animated = slides.overrideAttrs (oldAttrs: {
          pname = "rust-lecture-slides-animated";
          buildPhase = ''
            latexmk -pdf main-animated.tex
          '';
          installPhase = ''
            mkdir $out
            mv main-animated.pdf $out/main-animated.pdf
          '';
        });
        slides-all = pkgs.symlinkJoin {
          name = "rust-lecture-slides-all";
          paths = [
            slides
            slides-animated
          ];
        };
      in
      {
        packages = {
          inherit
            week1
            slides
            slides-animated
            slides-all
            ;
        };

        devShells = {
          rust = craneLib.devShell { };
          latex = pkgs.mkShell {
            packages = [ pkgs.texlive.combined.scheme-full ];
            shellHook = ''
              echo "devshell for compiling latex slides"
              echo "=========================="
              echo "Helpful commands:"
              echo "- build on file change: latexmk -pvc -pdf main.tex"
              echo "- build once: latexmk -pdf main.tex"
              echo "- cleanup: latexmk -C"
            '';
          };
        };
      }
    );
}
