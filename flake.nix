{
  description = "A repository management tool";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = system: nixpkgs.legacyPackages.${system};
    in
    {
      packages = forAllSystems (system: {
        default = (pkgsFor system).rustPlatform.buildRustPackage {
          pname = "rhq";
          version = "0.4.0-dev";
          src = ./.;
          cargoHash = "sha256-FOa+AUm9oBjMonJIrBgmV7PqiuzYA5UA9Jt7+PMRMc8=";
        };
      });
      devShells = forAllSystems (system: {
        default = (pkgsFor system).mkShell {
          buildInputs = with (pkgsFor system); [
            cargo
            rustc
            rust-analyzer
          ];
        };
      });
    };
}
