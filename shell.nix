{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rust-analyzer
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
}