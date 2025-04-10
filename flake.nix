{
  description = "A very basic flake";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      arch = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${arch};
    in {
      devShells.${arch}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer

          zellij
        ];

        shellHook = ''
          export SHELL="${pkgs.bashInteractive}/bin/bash";
          echo $SHELL;
        '';
      };
    };
}
