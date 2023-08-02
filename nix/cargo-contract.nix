{ pkgs, crane, system, version, rev, sha256 }:
  let 
    src = pkgs.fetchFromGitHub {
      owner = "paritytech";
      repo = "cargo-contract";
      rev = rev;
      sha256 = sha256;
    };


    rust-toolchain = pkgs.rust-bin.nightly."2023-01-01".default.override {
      extensions = [ "rust-src" ];
      targets = [ "wasm32-unknown-unknown" ];
    };

    craneLib = crane.lib.${system}.overrideToolchain rust-toolchain; 
  in
    craneLib.buildPackage {
      pname = "cargo-contract";
      version = version;
      src = src;
      doCheck = false;
      buildInputs = with pkgs; [
        cmake
      ];
    }




  
