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

  tasks = {
    "check:check".exec = "cargo check";
    "check:lint" = {
      exec = "cargo clippy";
      after = ["check:check"];
      before = ["run:debug"];
    };
    "run:debug" = {
      exec = "cargo run --features debug";
    };
    "run:release" = {
      exec = "cargo run --release";
    };
  };

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
