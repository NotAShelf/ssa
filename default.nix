{
  lib,
  rustPlatform,
}: let
  pname = "ssa";
  version = "0.1.0";
in
  rustPlatform.buildRustPackage {
    inherit pname version;
    src = builtins.path {
      name = "${pname}-${version}";
      path = ./.;
      filter = lib.cleanSourceFilter;
    };

    cargoLock.lockFile = ./Cargo.lock;

    meta = {
      description = "Simple, streamlined and pretty aggregator for systemd-analyze security";
      homepage = "https://github.com/notAShelf/ssa";
      license = lib.licenses.mit;
      maintainers = with lib.maintainers; [NotAShelf];
    };
  }
