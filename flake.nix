{
  description = "advent-of-code dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
        inherit (meta) name version;

        # Filter inputs to only those necessary for the build
        inputTexts = path: _type: builtins.match ".*[txt|md]$" path != null;
        inputTextsOrCargo = path: type:
          (inputTexts path type) || (craneLib.filterCargoSources path type);


        craneLib = crane.lib.${system};

        # Common derivation arguments used for all builds
        commonArgs = {
          src = nixpkgs.lib.cleanSourceWith {
            src = craneLib.path ./.; # The original, unfiltered source
            filter = inputTextsOrCargo;
          };
          strictDeps = true;

          buildInputs = [
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          nativeBuildInputs = with pkgs; [
            # Add extra native build inputs here, etc.
            # pkg-config
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          # Additional arguments specific to this derivation can be added here.
          # Be warned that using `//` will not do a deep copy of nested
          # structures
          pname = "aoc-deps";
        });

        # Run clippy (and deny all warnings) on the crate source,
        # reusing the dependency artifacts (e.g. from build scripts or
        # proc-macros) from above.
        #
        # Note that this is done as a separate derivation so it
        # does not impact building just the crate by itself.
        advent-of-code-clippy = craneLib.cargoClippy (commonArgs // {
          # Again we apply some extra arguments only to this derivation
          # and not every where else. In this case we add some clippy flags
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        advent-of-code = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        # Also run the crate tests under cargo-tarpaulin so that we can keep
        # track of code coverage
        advent-of-code-coverage = craneLib.cargoTarpaulin (commonArgs // {
          inherit cargoArtifacts;
        });

        # Audit licenses
        advent-of-code-deny = craneLib.cargoDeny (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          inherit advent-of-code advent-of-code-clippy advent-of-code-coverage advent-of-code-deny;
        };

        packages.default = advent-of-code;

        packages.advent-of-code = advent-of-code;

        apps.default = flake-utils.lib.mkApp {
          drv = advent-of-code;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
          packages = [
            pkgs.aoc-cli
          ];
        };
        
        packages.docker =
          let
            bin = "${advent-of-code}/bin/${name}";
          in
          pkgs.dockerTools.buildLayeredImage {
            inherit name;
            tag = "v${version}";

            config = {
              Entrypoint = [ bin ];
              ExposedPorts."8080/tcp" = { };
            };
          };
      });
}
