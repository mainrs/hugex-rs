{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix";
    cargo2nix.inputs = {
      flake-utils.follows = "flake-utils";
      nixpkgs.follows = "nixpkgs";
      rust-overlay.follows = "rust-overlay";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.67.1";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "clippy" "rustfmt" ];
        };

        workspaceShell = rustPkgs.workspaceShell {
          packages = with pkgs; [
            # CI.
            cargo-audit
            cargo-auditable
            cargo-deny
            codespell
            eclint

            # Development.
            cargo-expand
            cargo-watch
          ];
        };
      in rec {
        devShell = workspaceShell;
      }
    );
}
