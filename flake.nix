{
  description = "💥 A blazingly fast and feature-rich tool to auto theme and rice everything! based on wallpaper/image colors | written in Rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        ./default.nix
      ];
      systems = ["x86_64-linux" "aarch64-linux"];
    };
}
