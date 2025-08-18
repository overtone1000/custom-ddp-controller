# default.nix
with import <nixpkgs> {};

let
    #Define system libraries to be available on a path via environment variable for dlopen, which is used by winit
    #It does not seem these need to be included in the build inputs below. Would need to be confirm for deploying.
    driverLibPath = with pkgs; lib.makeLibraryPath [
    ];
  in {
    devShell = with pkgs; mkShell {
        name = "dev-environment";
        buildInputs = [
            #rust
            llvmPackages.bintools
            clang
            cargo
            rustc
            rustup
        ];
    };
  }