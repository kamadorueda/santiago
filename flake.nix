{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs: let
    nixpkgsForHost = host:
      import inputs.nixpkgs {system = host;};

    nixpkgs."x86_64-linux" = nixpkgsForHost "x86_64-linux";
  in {
    devShells."x86_64-linux".default = with nixpkgs."x86_64-linux";
      mkShell {
        name = "santiago";
        packages = [
          cargo
          cargo-tarpaulin
          clippy
          entr
          jq
          linuxPackages_latest.perf
          reuse
          rustc
        ];
      };

    apps."x86_64-linux".dev = with nixpkgs."x86_64-linux"; {
      type = "app";
      program =
        (writeShellScript "license" ''
          git ls-files | entr sh -euc '
            UPDATE=1 cargo test
            cargo doc
            cargo tarpaulin -o html
          '
        '')
        .outPath;
    };

    apps."x86_64-linux".license = with nixpkgs."x86_64-linux"; {
      type = "app";
      program =
        (writeShellScript "license" ''
          copyright='Kevin Amado <kamadorueda@gmail.com>'
          license='GPL-3.0-only'

          reuse addheader \
            --copyright="$copyright" \
            --license="$license" \
            --explicit-license \
            .envrc \
            Cargo.lock \
            Cargo.toml \
            tests/*/cases/*/* \
            tests/*/grammar_with_value.rs \
            tests/*/grammar.rs \
            tests/*/lexer.rs \
            flake.nix \
            flake.lock \

          git ls-files | xargs reuse addheader \
            --copyright="$copyright" \
            --license="$license" \
            --skip-unrecognised
        '')
        .outPath;
    };
  };
}
