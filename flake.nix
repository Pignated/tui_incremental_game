{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    my-repo.url = "github:pignated/flakes";
  };

  outputs = { self, nixpkgs, my-repo }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      myPackage = pkgs.callPackage (my-repo + "/rust/default.nix") { };
    in
    {
      packages.x86_64-linux.default = myPackage;
      devShells.x86_64-linux.default = pkgs.mkShell {
        inputsFrom = [ myPackage ];
        shellHook = myPackage.shellHook or "";
      };
    };
}
