{
  description = "A simple core lightning plugin that collect logs with opentelemetry";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
       pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

       clightning = pkgs.clightning.overrideAttrs (oldAttrs: {
          version = "master-log-notification";
          src = pkgs.fetchgit {
            url = "https://github.com/vincenzopalazzo/lightning";
            rev = "4fc9e71929f1a982c2b6a6059a2cd15989195da9";
            sha256 = "sha256-Ejpqbw002MCZ4kPmG/keoco6xdjCH/I3ICI5OA3HXEA=";
            fetchSubmodules = true;
          };
          configureFlags = [ "--disable-rust" "--disable-valgrind" ];
       });
      in {
        packages = {
          default = pkgs.gnumake;
        };
        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # build dependencies
            libcap
            gcc
            pkg-config
            git

            gnumake

            rustc
            cargo

            # integration test dependencies
            clightning
            bitcoind
          ];

          shellHook = ''
            export HOST_CC=gcc
            export PWD="$(pwd)"
          '';
        };
      }
    );
}
