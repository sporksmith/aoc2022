let
  sources = import ./nix/sources.nix { };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # debugging
    wabt

    # rust
    rustup

    # https://rustwasm.github.io/docs/book/game-of-life/setup.html
    wasm-pack
    cargo-generate
    nodejs_latest

    # More build deps
    glibc
    openssl
    openssl.dev
  ];
  shellHook = ''
    # Workaround: https://github.com/webpack/webpack/issues/14532#issuecomment-947525539
    export NODE_OPTIONS="--openssl-legacy-provider";

    # Workaround: https://github.com/NixOS/nixpkgs/issues/112535#issuecomment-1328173640
    export RUSTUP_HOME="$PWD/.rustup";
  '';
}
