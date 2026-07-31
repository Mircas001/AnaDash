# AnaDash - An dashboard for your PC
**THIS IS WIP! NOT TESTED YET!**
![InsertThumbnail](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/assets/thumbnail.png)
This is an dashboard that aims to have all the important functions and information within your arm's reach!

## Features:
- 4x Analog Gauges showing CPU usage, CPU temperature, RAM usage and Swap usage, driven by an MCP4728 chip!
- 8x Cherry MX keys for any macro your heart wishes!
- An LCD display that can show time, notifications and current song!
- An rotary encoder for changing your volume! Including a switch for mute!

## Cad Model
(insert cad here) FIXME

## PCB
This project was designed in KiCad, it uses a pair of 2 layer PCBs, the PCBs were split because the switches had to be at an angle while the pico had to sit straight.
For this to work, you must connect the following, all of the cables are JST-XH, as it features an locking mechanism.
You must make the following connections: (Pinouts are in left to right, when viewed from abovee!, GND will always be the rounded rectangle!) 
- DisplayConnector (BL, SCK, SDA, CS, RESET, CS, VCC, GND) - connect this to the matching pins on the display.
- LeftKeyConnector (Switch1, Switch2, Switch3, Switch4, GND) - The Input Board has matching connections!
- RightKeyConnector (Switch5, Switch6, Switch7, Switch8, GND) - The Input Board has matching connections!
- EncoderConnector (B, A, Switch, GND) - The Input Board has matching connections!

### Main PCB
This Board features the Pico, the DAC, the outputs for the ammeters, the display connector and the connections to the input board.            
[You can check out the PCB and schematic on KiCanvas!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fhardware%2FmainBoard)                                            
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/assets/mainBoardModel.png)

### Input PCB
This PCB has all the keys and the encoder and serves to take inputs, it is angled together with the display and gauges at 45 degrees for ergonomics!                 
[You can check out the PCB and schematic on KiCanvas!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fhardware%2FinputBoard)                                         
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/assets/inputBoardModel.png)

## Software
The software has been done entirely in rust, it's memory safe, professional, and has great performance!

Before setting up the software, you'll need to install these programs!
- [rustup](https://rustup.rs) - this is the rust toolchain, and it's the recommended way to compile rust code!
- [probe-rs](https://probe.rs) - **ONLY IF** you are gonna upload via an **debug probe**.
- [picotool](https://github.com/raspberrypi/pico-sdk-tools/releases) - **ONLY IF** you are gonna upload via **USB**.
- [lm-sensors](https://github.com/lm-sensors/lm-sensors) - This provides the sensors the driver can read from! Make sure to set it up before!

It's consisted in 3 parts:
- Firmware: Runs the Pico!
- Driver: Gets the info from your PC!
- Shared library: This is so both firmware and hardware agree with each other!

### Firmware
This firmware supports the Raspberry Pi Pico 1!
There are two ways you can upload it to the pico:

#### Upload via debug probe
For this, you need to connect your debug probe to SWD port!
```bash
git clone https://github.com/Mircas001/AnaDash.git
cd AnaDash/software/firmware
cargo run --release
```

#### Upload via USB
For this, all you need to do is connect the Raspberry Pi Pico to your computer via USB, and run with the usb-deploy feature!
```bash
git clone https://github.com/Mircas001/AnaDash.git
cd AnaDash/software/firmware
cargo run -F usb-deploy --release 
```

### Drivers
This only supports Linux for now! But I can answer questions about the code to help anyone who wants to port it! It works as a systemd service that uses udev rules to start as soon as you plug it in!

#### PKGBUILD method
If you run Arch Linux or any other Arch Based distro, you can use this to install! For now you have to manually install the PKGBUILD, but it'll get uploaded to the AUR once it's done!
**This is the recommended method.**
```bash 
git clone https://github.com/Mircas001/anadash-driver.git
cd anadash-driver
makepkg -si
```

#### Compile it yourself
This is more complicated! But it should work!
1. Make sure you are in the dialout group to get serial perms!
```bash
sudo usermod -aG dialout $USER
```
2. Install all the files!
```bash
git clone https://github.com/Mircas001/AnaDash.git
cd AnaDash/software/driver
cargo build --release
sudo cp target/release/anadash-driver /usr/local/bin
sudo chmod 755 /usr/local/bin/anadash-driver 
sudo cp anadash-driver@.service /etc/systemd/system
sudo cp 99-anadash.rules /etc/udev/rules.d
```
3. Reload everything!
```bash 
sudo systemctl daemon-reload
sudo udevadm control --reload-rules
sudo udevadm trigger
```
It should work as soon as you plug in the device!

You're also always welcome to implement these steps in an package file, so other people with your distro have it easier!

### Troubleshooting the drivers
Since it's an systemd service, you can get the logs like this, replacing ```<serialport>``` with the device's serial port! Usually, it's ttyUSB0.
```bash
systemctl status anadash-driver@<serialport>.service
journalctl -u anadash-driver@<serialport>.service
```

## Debugging the device
There is an UART port at the main board for debugging!
Also, there are also some key combinations baked in the firmware:
- You can press the 1st key and the 8th key (the two outhermost keys) to reset the pico!
- You can press the 4th key and the 5th key (the two innermost keys) to go into DFU mode!

## Miscellaneous note
You will need to print the labels for the meters in A4 sticker paper, if you are an manufacturer, you should get it custom printed!                           
The labels are available here: [Labels](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/production/meterLabels.svg)

There is an LED header added to the PCB, however, I did not implement the code for it, it is there for future use!

## BOM
You can get the master BOM here, which contains all parts necessary to the project: [BOM](https://github.com/Mircas001/AnaDash/blob/main/production/BOM.csv)

You can also get the JLCPCB BOM for the main board (in case you want to get PCBA) here: [BOM](https://github.com/Mircas001/AnaDash/blob/main/production/mainBoard/jlcpcb_bom.csv)

However, if you do get PCBA, it's recommended to only request the soldering of SMD components, as it's cheaper this way! Also, the 665ohm resistors need to be manually filled in, apparently they're so new there's not even a footprint for it! But I believe this issue will go away with time.         

The input board does not feature an JLCPCB BOM for PCBA because they do not have the parts for it! However, I have left the netlist and positions there in case you have an pick and place machine, but official support for assembling it this way is NOT provided! It's recommended to hand solder instead! Only THD components are used there.

## Software used:
This project was designed in:
- [KiCad for the PCBs!](https://www.kicad.org)
- [Visual Studio Code for the software!](https://code.visualstudio.com/)
- [Onshape for the 3D design!](https://onshape.com/)

## TODO List:
- [X] Design the main PCB 
- [X] Design the input PCB
- [X] Design the meters
- [X] Make the drivers
- [X] Make the driver into an actual driver
- [X] Make the firmware
- [ ] Make the Case

## Credits
- [Lex Bailey, for giving me the inspiration to start this project a few years ago, and for help with the meter labels](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
- [Hack Club Macondo program, for the funding](https://macondo.hackclub.com/)
- [OrpheusPad, for serving as reference as to how should the git repo and README look like](https://github.com/qcoral/orpheuspad/tree/main)
- [HackPad program, which, despite not participating in it, guided me through PCB design](https://hackpad.hackclub.com/)
- [CarlKCarlK's clock project which I'm using as inspiration for parts of the codecode](https://github.com/CarlKCarlK/clock)
- [Siliconwit's guide on embassy-usb](https://siliconwit.com/education/embedded-rust-rp2040/usb-device-embassy/)
