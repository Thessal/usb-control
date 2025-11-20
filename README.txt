Re:Speaker LED control example

* Installation
let 
  pkgs = import <nixos> { };
  respeaker-led-off = pkgs.callPackage (builtins.fetchGit {
    url = "https://github.com/Thessal/usb-control";
    ref = "main";
    rev = "d87f939999af17d8a1eb6b554ec19b8f1465b2d8";
  }) {};

* Usage 
respeaker-led-off

* Configuration.nix can be modified for normal user usb accsss privilege
1) Set suid
- /etc/nixos/respeaker.nix
{ pkgs, ... }: 
let respeaker-led-off = pkgs.callPackage (builtins.fetchGit {
    url = "https://github.com/Thessal/usb-control";
    ref = "main";
    rev = "06fa2c468816f1411a1d49d7220f7b0c30ddc548";
  }) {};
in { 
  environment.systemPackages = [ respeaker-led-off ];
  security.wrappers.respeaker-led-off = {
    source = "${respeaker-led-off}/bin/respeaker-led-off";
    owner = "root";
    group = "root";
    setuid = true;
  };
}
- /etc/nixos/configuration.nix
imports =
[ # Include the results of the hardware scan.
    ./hardware-configuration.nix
    ./respeaker.nix
];
2) Allow user to access usb (Does not work)
  services.udev.extraRules = ''
    SUBSYSTEM=="usb", ATTRS{idVendor}=="2886", ATTRS{idProduct}=="0018", MODE="0666"
  '';
