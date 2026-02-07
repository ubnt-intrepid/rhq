{
  description = "A repository management tool";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rustPlatform;
      in
      {
        packages.default = rust.buildRustPackage {
          pname = "rhq";
          version = "0.4.0-dev";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git # Needed for cargo-init and other Git operations within the shell.
            # pkg-config openssl ...
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "clippy" "rustfmt" ];
            })
          ];
          RUST_SRC_PATH = rust.rustLibSrc; # Ensure rust-analyzer can find the source code.
        };
      }
    );
}
