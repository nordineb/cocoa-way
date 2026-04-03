# ABOUTME: Nix flake for reproducible cocoa-way development environment.
# ABOUTME: Provides all system deps, Rust toolchain, and CI tools in a locked devShell.
{
  description = "cocoa-way — native macOS Wayland compositor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachSystem [ "aarch64-darwin" ] (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain
            rustToolchain

            # System libraries required by cocoa-way
            pkgs.libxkbcommon
            pkgs.pixman
            pkgs.pkg-config

            # CI/dev tools
            pkgs.just
            pkgs.cargo-audit
            pkgs.cargo-deny
          ];

          shellHook = ''
            echo "cocoa-way dev shell loaded"
            echo "  rust: $(rustc --version)"
            echo "  just: $(just --version)"
            echo ""
            echo "Run 'just --list' to see available tasks"
          '';

          # Ensure pkg-config can find nix-provided libraries
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" [
            pkgs.libxkbcommon.dev
            pkgs.pixman
          ];
        };
      }
    );
}
