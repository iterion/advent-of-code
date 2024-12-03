#{
#  description = "advent-of-code dev environment";
#
#  inputs = {
#    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#
#    zig.url = "github:mitchellh/zig-overlay";
#
#    crane = {
#      url = "github:ipetkov/crane";
#      inputs.nixpkgs.follows = "nixpkgs";
#    };
#
#    flake-utils.url = "github:numtide/flake-utils";
#  };
#
#  outputs = { self, nixpkgs, zig, crane, flake-utils, ... }:
#    flake-utils.lib.eachDefaultSystem (system:
#      let
#        pkgs = nixpkgs.legacyPackages.${system};
#
#        meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
#        inherit (meta) name version;
#
#        # Filter inputs to only those necessary for the build
#        inputTexts = path: _type: builtins.match ".*[txt|md]$" path != null;
#        inputTextsOrCargo = path: type:
#          (inputTexts path type) || (craneLib.filterCargoSources path type);
#
#
#        craneLib = crane.lib.${system};
#
#        # Common derivation arguments used for all builds
#        commonArgs = {
#          src = nixpkgs.lib.cleanSourceWith {
#            src = craneLib.path ./.; # The original, unfiltered source
#            filter = inputTextsOrCargo;
#          };
#          strictDeps = true;
#
#          buildInputs = [
#          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
#            # Additional darwin specific inputs can be set here
#            pkgs.libiconv
#          ];
#
#          nativeBuildInputs = with pkgs; [
#            # Add extra native build inputs here, etc.
#            # pkg-config
#          ];
#
#          # Additional environment variables can be set directly
#          # MY_CUSTOM_VAR = "some value";
#        };
#
#        # Build *just* the cargo dependencies, so we can reuse
#        # all of that work (e.g. via cachix) when running in CI
#        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
#          # Additional arguments specific to this derivation can be added here.
#          # Be warned that using `//` will not do a deep copy of nested
#          # structures
#          pname = "aoc-deps";
#        });
#
#        # Run clippy (and deny all warnings) on the crate source,
#        # reusing the dependency artifacts (e.g. from build scripts or
#        # proc-macros) from above.
#        #
#        # Note that this is done as a separate derivation so it
#        # does not impact building just the crate by itself.
#        advent-of-code-clippy = craneLib.cargoClippy (commonArgs // {
#          # Again we apply some extra arguments only to this derivation
#          # and not every where else. In this case we add some clippy flags
#          inherit cargoArtifacts;
#          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
#        });
#
#        # Build the actual crate itself, reusing the dependency
#        # artifacts from above.
#        advent-of-code = craneLib.buildPackage (commonArgs // {
#          inherit cargoArtifacts;
#        });
#
#        # Also run the crate tests under cargo-tarpaulin so that we can keep
#        # track of code coverage
#        advent-of-code-coverage = craneLib.cargoTarpaulin (commonArgs // {
#          inherit cargoArtifacts;
#        });
#
#        # Audit licenses
#        advent-of-code-deny = craneLib.cargoDeny (commonArgs // {
#          inherit cargoArtifacts;
#        });
#      in
#      {
#        checks = {
#          inherit advent-of-code advent-of-code-clippy advent-of-code-coverage advent-of-code-deny;
#        };
#
#        packages.default = advent-of-code;
#
#        packages.advent-of-code = advent-of-code;
#
#        apps.default = flake-utils.lib.mkApp {
#          drv = advent-of-code;
#        };
#
#        devShells.default = craneLib.devShell {
#          # Inherit inputs from checks.
#          checks = self.checks.${system};
#
#          # Additional dev-shell environment variables can be set directly
#          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
#          packages = [
#            pkgs.aoc-cli
#          ];
#        };
#        
#        packages.docker =
#          let
#            bin = "${advent-of-code}/bin/${name}";
#          in
#          pkgs.dockerTools.buildLayeredImage {
#            inherit name;
#            tag = "v${version}";
#
#            config = {
#              Entrypoint = [ bin ];
#              ExposedPorts."8080/tcp" = { };
#            };
#          };
#      });
#}
{
  description = "aoc development environment";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
    zig.url = "github:mitchellh/zig-overlay";
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay, zig }:
    let
      # Overlays enable you to customize the Nixpkgs attribute set
      overlays = [
        # Makes a `rust-bin` attribute available in Nixpkgs
        (import rust-overlay)

        # Makes a `zig` attribute available in Nixpkgs
        (final: prev: {
          zigpkgs = zig.packages.${prev.system};
        })

        # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
        # create a Rust environment
        (self: super: {
          rustToolchain = super. rust-bin.stable.latest.default.override {
            extensions = [ "rustfmt" "llvm-tools-preview" ];
          };
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
        pkgs = import nixpkgs { inherit overlays system; config.allowBroken = true; };
      });

    in
    {
      # Development environment output
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          # The Nix packages provided in the environment
          packages = (with pkgs; [
            # The package provided by our custom overlay. Includes cargo, Clippy, cargo-fmt,
            # rustdoc, rustfmt, and other tools.
            rustToolchain

            pkgs.zigpkgs.master

            # dependencies for aoc
            just
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            libiconv 
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ]);
        };
      });
    };
}
