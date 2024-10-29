{
  lib,
  rustPlatform,
  stdenvAdapters,
  llvm,
}: let
  toml = (lib.importTOML ../Cargo.toml).package;
  pname = toml.name;
  inherit (toml) version;
in
  rustPlatform.buildRustPackage.override {stdenv = stdenvAdapters.useMoldLinker llvm.stdenv;} {
    inherit pname version;
    src = builtins.path {
      name = "${pname}-${version}";
      path = ../.;
      filter = lib.cleanSourceFilter;
    };

    RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
    cargoLock.lockFile = ../Cargo.lock;

    meta = {
      description = "Simple, streamlined and pretty aggregator for systemd-analyze security";
      homepage = "https://github.com/notAShelf/ssa";
      license = lib.licenses.mit;
      maintainers = with lib.maintainers; [NotAShelf];
    };
  }
