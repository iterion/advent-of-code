{
  description = "api-deux development environment";

  # Flake inputs
  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay }:
    let
      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      inherit (meta) name version;

      overlays = [
        # Makes a `rust-bin` attribute available in Nixpkgs
        (import rust-overlay)
        # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
        # create a Rust environment
        (self: super: {
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];

      # Systems supported
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };

        # runCiLocally = builtins.writeScriptBin "ci-local" ''
        #   echo "Checking Rust formatting..."
        #   cargo fmt --check

        #   echo "Auditing Rust dependencies..."
        #   cargo-deny check

        #   echo "Auditing editorconfig conformance..."
        #   eclint -exclude "Cargo.lock"

        #   echo "Checking spelling..."
        #   codespell \
        #     --skip target,.git \
        #     --ignore-words-list crate

        #   echo "Testing Rust code..."
        #   cargo test

        #   echo "Building bin..."
        #   nix build .#advent-of-code
        # '';
      });
    in
    {
      # Development environment output
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          packages = (with pkgs; [
            rustToolchain
            aoc-cli
            pkg-config
            openssl
            cargo-deny

            cargo-edit
            cargo-watch

            # Spelling and linting
            codespell
            eclint
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            libiconv 
            darwin.apple_sdk.frameworks.Security
          ]);
        };
      });

      packages = forAllSystems({ pkgs }: rec {
        default = advent-of-code;

        advent-of-code = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          inherit version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          release = true;
        };
      });
    };
}
