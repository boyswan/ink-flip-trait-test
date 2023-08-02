
{
                                                                           
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
   rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, crane, fenix, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
        ];

        pkgs = import nixpkgs {
          inherit overlays system; 
          config.allowUnfree = true;
        };

        rust-toolchain = pkgs.rust-bin.nightly."2023-02-04".default.override {
          extensions = [ "rust-src" "rust-analyzer"];
          targets = [ "wasm32-unknown-unknown" ];
        };

        cargo-contract = import ./nix/cargo-contract.nix {
          inherit pkgs system crane;
          version = "3.0.1";
          rev = "c746b0d373197edd4dbe0fb444f24f0ac278ef11";
          sha256 = "sha256-4PeEq1iAZPm90hcAJnM5B6Bwj24vSc3X1BGcUB109n0=";
        };
        

      in {
        devShells.default = with pkgs; mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            cargo-contract
          ];
        };
      });
}
