{
  perSystem = {
    pkgs,
    lib,
    ...
  }: {
    packages.default = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
      pname = "wallrust";
      version = "v1.0.5";

      src = pkgs.fetchFromGitHub {
        owner = "prime-run";
        repo = "wallrust";
        hash = "sha256-PVHtpj3Vc7dJWnbLnvCGMdmOMlvGRet6bKLswxOnAcw";
        tag = finalAttrs.version;
      };

      cargoHash = "sha256-R2RjWCDUh60LN7gy4oWqBEDFft07jY3J654MpAnv/es";

      meta = {
        description = "💥 A blazingly fast and feature-rich tool to auto theme and rice everything! based on wallpaper/image colors | written in Rust";
        homepage = "https://github.com/prime-run/wallrust";
        license = lib.licenses.mit;
      };
    });
  };
}
