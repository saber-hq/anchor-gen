{
  description = "Standard Solana development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.nixfmt-rfc-style;
        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo-workspaces
              rustup
              anchor
              cargo-expand
              cargo-readme
            ];
          };
      }
    );
}
