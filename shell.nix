let
  sources = import ./nix/sources.nix { };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # basics. maybe just inherit instead?
    #coreutils
    #git
    #vim

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
  ];
  # Workaround: https://github.com/webpack/webpack/issues/14532#issuecomment-947525539
  NODE_OPTIONS = "--openssl-legacy-provider";
}
