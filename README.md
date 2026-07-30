# AnaDash - An dashboard for your PC
# THIS IS WIP! NOT TESTED YET!
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
- LeftKeyConnector (Switch1, Switch2, Switch3, Switch4, GND) - connect this to the matching connector in the auxiliary board 
- RightKeyConnector (Switch5, Switch6, Switch7, Switch8, GND) - Connect this to the right connector in the auxiliary board
- EncoderConnector (B, A, Switch, GND) - Connect this to the center connector in the input board!

### Main PCB
This Board features the Pico, the DAC, the outputs for the ammeters, the display connector and the connections to the input board                               
[You can check out the PCB and schematic on KiCanvas!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fhardware%2FmainBoard)
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/assets/mainBoardModel.png)

### Input PCB
This PCB has all the keys and the encoder and serves to take inputs, it is angled together with the display and gauges at 45 degrees for ergonomics!                 
[You can check out the PCB and schematic on KiCanvas!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fhardware%2FinputBoard)
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/assets/inputBoardModel.png)

## Software
The software has been done entirely in rust, it's memory safe, professional, and has great performance!
So, before setting up the software, you'll need to install these programs! Also make sure to get it from your package manager!
- [rustup](https://rustup.rs) - this is the rust toolchain, and it's the recommended way to compile rust code!
- [probe-rs](https://probe.rs) - ONLY IF you are gonna upload using an debug probe
- [picotool](https://github.com/raspberrypi/pico-sdk-tools/releases) - ONLY if you are gonna upload via USB  

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
This only supports Linux for now! But I can answer questions about the code to help anyone who wants to port it!
You need to have lm-sensors installed and set up for this to work!
It works as a systemd service that uses udev rules to start as soon as you plug it in!
#### PKGBUILD method
If you run Arch Linux, you can use this to install! For now you have to manually install the PKGBUILD, but it'll get uploaded to the AUR once it's done!
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
- You can press key1 and key8 (the two outhermost keys) to reset the pico!
- You can press key4 and key5 (the two innermost keys) to go into DFU mode!

## BOM
You can get the master BOM here, which contains all parts necessary to the project: [BOM](https://github.com/Mircas001/AnaDash/blob/main/production/MiscBOM.csv)
You can also get the BOM for the main board (in case you want to get PCBA) here: [BOM](https://github.com/Mircas001/AnaDash/blob/main/production/mainBoard/bom.csv)
However, if you do get PCBA, make sure that:
1. You only request they solder the SMD components
2. You will need to manually fill in the 665ohm resistor, apparently, they're new, so it's not accepting it right out of the bat.
Also, there is an BOM for the input board, but I did not put in the LCSC parts because they don't have them, and I do not recommend you use it, since it's all through hole components. You should order the parts from aliexpress and manually solder it yourself instead!

## Software used:
This project was designed in:
[KiCad](https://www.kicad.org) (for the PCB)

## TODO List:
- [X] Design the main PCB 
- [X] Design the input PCB
- [X] Design the meters
- [X] Make the drivers
- [X] Make the driver into an actual driver
- [ ] Make the firmware
- [ ] Make the Case

## Credits
[Lex Bayley, for giving me the inspiration to start this project a few years ago, and for help designing the meter labels](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
[Hack Club Macondo program, for the funding:](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
[OrpheusPad, for serving as reference as to how should the git repo and README look like](https://github.com/qcoral/orpheuspad/tree/main)
[HackPad program, which, despite not participating in it, guided me through this](https://hackpad.hackclub.com/)
[CarlKCarlK's clock project which I'm using as inspiration as well as some of his code](https://github.com/CarlKCarlK/clock)
[Cescentro guide on how to write an driver!](https://crescentro.se/posts/writing-drivers/)
[Siliconwit's guide on embassy-usb](https://siliconwit.com/education/embedded-rust-rp2040/usb-device-embassy/)
