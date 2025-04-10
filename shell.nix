{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    rustup
    cargo
    rustc
    napi-rs-cli
  ];

  buildInputs = [
    pkgs.bashInteractive
  ];

  shellHook = ''
    export RUST_SRC_PATH=$(which rustc)
  '';
}
