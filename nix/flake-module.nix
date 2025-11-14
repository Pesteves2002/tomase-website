{
  self,
  lib,
  inputs,
  flake-parts-lib,
  ...
}: let
  inherit
    (flake-parts-lib)
    mkPerSystemOption
    ;
in {
  options = {
    perSystem =
      mkPerSystemOption
      ({
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        options = {
          leptos-fullstack.overrideCraneArgs = lib.mkOption {
            type = lib.types.functionTo lib.types.attrs;
            default = _: {};
            description = "Override crane args for the leptos-fullstack package";
          };

          leptos-fullstack.rustToolchain = lib.mkOption {
            type = lib.types.package;
            description = "Rust toolchain to use for the leptos-fullstack package";
            default = (pkgs.rust-bin.fromRustupToolchainFile (self + /rust-toolchain.toml)).override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "clippy"
              ];
            };
          };

          leptos-fullstack.craneLib = lib.mkOption {
            type = lib.types.lazyAttrsOf lib.types.raw;
            default = (inputs.crane.mkLib pkgs).overrideToolchain config.leptos-fullstack.rustToolchain;
          };

          leptos-fullstack.src = lib.mkOption {
            type = lib.types.path;
            description = "Source directory for the leptos-fullstack package";
            # When filtering sources, we want to allow assets other than .rs files
            # TODO: Don't hardcode these!
            default = lib.cleanSourceWith {
              src = self; # The original, unfiltered source
              filter = path: type:
                (lib.hasSuffix "\.html" path)
                ||
                # Example of a folder for images, icons, etc
                (lib.hasInfix "/public/" path)
                || (lib.hasInfix "/style/" path)
                ||
                # Default filter from crane (allow .rs files)
                (config.leptos-fullstack.craneLib.filterCargoSources path type);
            };
          };
        };

        config = let
          cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
          inherit (cargoToml.package) name version;
          inherit (config.leptos-fullstack) rustToolchain craneLib src;

          # Crane builder for cargo-leptos projects
          craneBuild = rec {
            args = {
              inherit src;
              pname = name;
              version = version;
              buildInputs = with pkgs; [
                cargo-generate
                cargo-leptos
                binaryen # Provides wasm-opt
                dart-sass
              ];
            };
            cargoArtifacts = craneLib.buildDepsOnly args;
            buildArgs =
              args
              // {
                inherit cargoArtifacts;
                doNotPostBuildInstallCargoBinaries = true;

                buildPhaseCargoCommand = ''
                  cargo leptos build --release -vvv
                '';
                # cargoTestCommand = "cargo leptos test --release -vvv";
                nativeBuildInputs = [
                  pkgs.makeWrapper
                ];
                installPhaseCommand = ''
                  mkdir -p $out/bin
                  ls target/
                  ls target/release
                  cp target/release/${name} $out/bin/
                  cp -r target/site $out/bin/
                  wrapProgram $out/bin/${name} \
                  --set LEPTOS_SITE_ROOT $out/bin/site
                '';
              };
            package = craneLib.buildPackage (buildArgs // config.leptos-fullstack.overrideCraneArgs buildArgs);

            check = craneLib.cargoClippy (args
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
              });

            doc = craneLib.cargoDoc (args
              // {
                inherit cargoArtifacts;
              });
          };

          rustDevShell = pkgs.mkShell {
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
              export RUSTFLAGS="--cfg erase_components";
            '';
            buildInputs = [
              pkgs.libiconv
            ];
            nativeBuildInputs = [
              rustToolchain
            ];
          };
        in {
          # Rust package
          packages.${name} = craneBuild.package;
          packages."${name}-doc" = craneBuild.doc;

          checks."${name}-clippy" = craneBuild.check;

          # Rust dev environment
          devShells.${name} = pkgs.mkShell {
            inputsFrom = [
              rustDevShell
            ];
            nativeBuildInputs = with pkgs; [
              cargo-generate
              dart-sass
              cargo-leptos
              binaryen # Provides wasm-opt
            ];
          };
        };
      });
  };
}
