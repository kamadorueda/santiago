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
          clippy
          jq
          rustc
        ];
      };
  };
}
