{
  description = "A Nix-flake-based Rust development environment";

    inputs = {
      nixpkgs = {
        type = "github";
        owner = "NixOS";
        repo = "nixpkgs";
        ref = "nixos-25.05";
      };
      rust-overlay = {
        url = "github:oxalica/rust-overlay";
        inputs.nixpkgs.follows = "nixpkgs";
      };
      flake-utils.url = "github:numtide/flake-utils";
    };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        packages = with pkgs; [
          cargo
          cargo-tauri
          rust-analyzer-unwrapped
          webkitgtk_4_1
          noto-fonts-color-emoji
          glib-networking
          pkg-config
          openssl
        ];

        nativeBuildPackages = with pkgs; [
          pkg-config
          dbus
          openssl
          glib
          gtk3
          libsoup_2_4
          webkitgtk_4_1
          librsvg
          noto-fonts-color-emoji
          wrapGAppsHook4
        ];

        libraries = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
        ];

      in {

        devShells.default = pkgs.mkShell {
          buildInputs = packages;

          nativeBuildInputs = nativeBuildPackages;

          env = {
            # OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib";
            # OPENSSL_NO_VENDOR=1;
            # OPENSSL_DIR="${pkgs.openssl.dev}";
            PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH";
          };

          shellHook = with pkgs; ''
            export LD_LIBRARY_PATH="${
              lib.makeLibraryPath libraries
            }:$LD_LIBRARY_PATH"

            export OPENSSL_INCLUDE_DIR="${openssl.dev}/include/openssl"

            export OPENSSL_LIB_DIR="${openssl.out}/lib"

            export OPENSSL_ROOT_DIR="${openssl.out}"

            export WEBKIT_DISABLE_DMABUF_RENDERER=1
          '';
        };
      });
}

# env = {
#             OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib";
#             OPENSSL_NO_VENDOR=1;
#             OPENSSL_DIR="${pkgs.openssl.dev}";
#             PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH";
#             RUSTFLAGS="-C target-cpu=native";
#             WEBKIT_DISABLE_DMABUF_RENDERER=1;
#             # Required by rust-analyzer
#             # RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
#           };