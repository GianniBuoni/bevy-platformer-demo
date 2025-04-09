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

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
