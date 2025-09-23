{ pkgs ? import <nixpkgs> { }, target_ip ? "10.10.30.17", target_port ? "4048", bind_port ? "30125", path ? [ pkgs.echo ], hyper_hash ? pkgs.lib.fakeHash, ... }:
#Ensure nixpkgs is up to date. Check the channel currently used with sudo nix-channel --list (it's the one named nixos) and the rustc version with rustc -V
#This requires git installed systemwide in environment.systemPackages. Build the system to install git, then rebuild to install this config.

let 
  repo = fetchGit {
    url = "https://github.com/overtone1000/custom-ddp-controller.git";
    ref = "main"; #this does seem to be necessary
    shallow = true;
    #rev = "4ebf990e1bedd27464f033f5dfd046a1ec610e43"; #sometimes need to force it to rebuild
  };

  #manifest = (pkgs.lib.importTOML ("${repo}/Cargo.toml")).package;
  core_manifest = (pkgs.lib.importTOML ("${repo}/core/Cargo.toml")).package;
  lock = ("${repo}/Cargo.lock");

  package = pkgs.rustPlatform.buildRustPackage {
    pname = core_manifest.name;
    version = core_manifest.version;
    
    src = "${repo}";

    #Using cargoLock instead
    #cargoHash = ""; #Determine correct checksum by attempting build and viewing error output

    cargoLock={
      lockFile = (lock);
      allowBuiltinFetchGit = true;
      #outputHashes = { #Not needed with allowBuiltinFetchGet
      #   "hyper-services-0.1.0" = hyper_hash;
      #};
    };
  };
in
{
  systemd.services.custom-ddp-controller = {
    wantedBy = ["multi-user.target"];
    wants = [ "network-online.target" ]; #For health checkin
    script = "${package}/bin/${core_manifest.name} ${target_ip} ${target_port} ${bind_port}";
    path = path;
    serviceConfig = {
      User = "root";
      Restart = "always";
    };
  };
}