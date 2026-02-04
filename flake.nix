{
  description = "Moxid flake";
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
      nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-23.11";
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux"; # your version
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [
          trunk
          cargo-tauri
          glib
          pkg-config
          librsvg
          lld
          xdg-utils
          webkitgtk_4_1
        ]; # whatever you need
      };
    };
}

