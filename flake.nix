{
  description = "Rumminator - A Contract Rummy AI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        rumminator = pkgs.rustPlatform.buildRustPackage {
          pname = "rumminator";
          version = "0.1.0";
          src = ./.;
          
          cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
          
          nativeBuildInputs = with pkgs; [
            rustToolchain
          ];

          meta = with pkgs.lib; {
            description = "A Contract Rummy AI using classical and reinforcement learning approaches";
            license = licenses.mit;
          };
        };

      in
      {
        packages = {
          default = rumminator;
          rumminator = rumminator;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
          ];

          shellHook = ''
            echo "🃏 Rumminator Development Environment"
            echo "Commands available:"
            echo "  rumminator-build  - Build the project"
            echo "  rumminator-run    - Run the project"
            echo "  rumminator-test   - Run tests"
            echo "  rumminator-watch  - Watch and rebuild on changes"
            echo ""
            
            # Create convenient aliases
            alias rumminator-build="cargo build"
            alias rumminator-run="cargo run"
            alias rumminator-test="cargo test"
            alias rumminator-watch="cargo watch -x run"
            
            # Set up environment
            export RUST_BACKTRACE=1
          '';
        };
      });
}