{rustPlatform ? (import <nixpkgs> {}).rustPlatform}:
rustPlatform.buildRustPackage rec {
  pname = "maze";
  version = "1.0.0";
  src = ./.;
  cargoSha256 = "sha256-4H+lOf4/r7uWnItGkhlBNGGKG2ppuOocAWvGj+7qakI=";
}

