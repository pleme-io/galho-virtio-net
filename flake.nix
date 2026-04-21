{
  description = "galho-virtio-net — brasa's virtio-net userspace driver";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachSystem [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = (fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = pkgs.lib.fakeSha256;
        });
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          name = "galho-virtio-net-dev";
          packages = [ rustToolchain ] ++ (with pkgs; [ cargo-nextest just ]);
        };
        packages.default = pkgs.writeTextFile {
          name = "galho-virtio-net-phase-0";
          text = "Phase 0 — skeleton only.\n";
          destination = "/STATUS";
        };
      }
    );
}
