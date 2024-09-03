{
  description =
    "A terminal spotify player that has feature parity with the official client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, ... }:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import (inputs.nixpkgs) { inherit system overlays; };
        inherit (pkgs) lib;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.minimal;
          rustc = pkgs.rust-bin.stable.latest.minimal;
        };

        nativeBuildInputs = with pkgs;
          [ pkg-config cmake rustPlatform.bindgenHook ]
          ++ lib.optionals stdenv.isDarwin [ makeBinaryWrapper ];

        buildInputs = with pkgs;
          [ openssl dbus fontconfig ] ++ lib.optionals stdenv.isLinux [
            libsixel
            alsa-lib
            libpulseaudio
            portaudio
            libjack2
            SDL2
            gst_all_1.gstreamer
            gst_all_1.gst-devtools
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
          ] ++ lib.optionals stdenv.isDarwin
          (with darwin.apple_sdk.frameworks; [
            AppKit
            AudioUnit
            Cocoa
            MediaPlayer
          ]);

        manifest = (lib.importTOML ../spotify_player/Cargo.toml).package;
      in {
        packages.default = rustPlatform.buildRustPackage {
          inherit buildInputs nativeBuildInputs;
          inherit (manifest) name version;

          src = ../.;

          cargoLock = {
            lockFile = ../Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          meta.mainProgram = "spotify_player";

          # TODO: Macos link sixel-sys to libsixel
          # postInstall = lib.optionals (stdenv.isDarwin && withSixel) ''
          #   wrapProgram $out/bin/spotify_player \
          #     --prefix DYLD_LIBRARY_PATH : "${lib.makeLibraryPath [ libsixel ]}"
          # '';
        };

        devShell = pkgs.mkShell {
          name = "spotify-player-shell";
          inherit nativeBuildInputs;

          buildInputs = buildInputs ++ (with pkgs.rust-bin; [
            (stable.latest.minimal.override {
              extensions = [ "clippy" "rust-src" ];
            })

            nightly.latest.clippy
            nightly.latest.rustfmt
            nightly.latest.rust-analyzer
          ]);
        };
      });
}
