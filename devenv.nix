{pkgs, ...}: {
  packages = with pkgs; [
    alsa-lib-with-plugins
    udev
    wayland
  ];

  env.LD_LIBRARY_PATH = with pkgs;
    lib.makeLibraryPath [
      libxkbcommon
      vulkan-loader
    ];

  scripts = {
    cr.exec = ''
      cargo clippy && cargo run
    '';
    crr.exec = ''
      cargo clippy && cargo run --release
    '';
  };

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
