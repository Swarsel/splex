# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {self, nixpkgs, rust-overlay, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  in {
    devShells.${system}.default = pkgs.mkShell {

      packages = with pkgs; [
        cargo
        clippy
        rustc
        rustfmt
        toolchain
        rust-analyzer-unwrapped
        rust-analyzer
        (pkgs.python3.withPackages (python-pkgs: [
          python-pkgs.numpy
          python-pkgs.pandas
          python-pkgs.scipy
          python-pkgs.matplotlib
          python-pkgs.networkx
          python-pkgs.debugpy
          python-pkgs.flake8
          python-pkgs.gnureadline
          python-pkgs.python-lsp-server
        ]))
      ];
      env = {
        RUST_BACKTRACE = "full";
      };
        RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

      # ...

    };
  };
}
