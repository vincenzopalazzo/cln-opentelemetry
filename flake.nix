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
            rev = "bfbf4da19db71ef8b506db547abf9abaa20b03c0";
            sha256 = "sha256-kVCd6rXNfPb8msM7PUC4g8lPpT15aHjFngXD8SNbWxE=";
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
