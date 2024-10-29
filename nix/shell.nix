{
  mkShell,
  rust-analyzer-unwrapped,
  rustfmt,
  clippy,
  cargo,
  rustc,
  rustPlatform,
}:
mkShell {
  name = "ssa";
  strictDeps = true;
  packages = [
    cargo
    rustc

    rust-analyzer-unwrapped
    rustfmt
    clippy
  ];

  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
