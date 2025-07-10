{
  pkgs,
  ...
}:

{
  dotenv.enable = true;

  packages = with pkgs; [
    git
    nixfmt-rfc-style
    rustup
  ];
}
