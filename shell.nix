{
  pkgs ? let
    lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
    nixpkgs = fetchTarball {
      url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
      sha256 = lock.narHash;
    };
  in
    import nixpkgs {overlays = [];},
  ...
}: let
  # Helpful nix function
  # getLibFolder = pkg: "${pkg}/lib";
  # Manifest via Cargo.toml
  # manifest = (pkgs.lib.importTOML ./Cargo.toml).workspace.package;
in
  pkgs.stdenv.mkDerivation {
    name = "abrt-dbus";

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      # GCC toolchain
      gcc
      gnumake
      pkg-config

      # LLVM toolchain
      cmake
      llvmPackages.llvm
      llvmPackages.clang

      # Hail the Nix
      nixd
      statix
      deadnix
      alejandra

      # Rust
      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
      cargo-watch

      # Databases & ORM
      # postgresql
      # diesel-cli
      # diesel-cli-ext

      # A few utils
      # just
    ];

    # Runtime dependencies which will be shipped
    # with nix package
    buildInputs = with pkgs; [
      openssl
      dbus
      pkg-config
      # libressl
    ];

    # Set Environment Variables
    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }
