# AnaDash - An dashboard for your PC
This is an dashboard that aims to have all the important functions and information within your arm's reach!

## Features:
- 4x Analog Gauges showing CPU usage, CPU temperature, RAM usage and Swap usage, driven by an Adafruit MCP4728 module!
- 8x Cherry MX keys for any macro your heart wishes!
- An OLED display that can show animations, time, notification and current song!
- An rotary encoder for UI navigation and changing your volume! Including a switch for mute!

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
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/hardware/images/mainBoardModel.png)

### Input PCB
This PCB has all the keys and the encoder and serves to take inputs, it is angled together with the display and gauges at 45 degrees for ergonomics!
[You can check out the PCB and schematic on KiCanvas!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fhardware%2FinputBoard)
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/hardware/images/inputBoardModel.png)

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
PKGBUILD method:
TODO: Insert PKGBUILD or something here
Compile it yourself:
```bash
git clone https://github.com/Mircas001/AnaDash.git
cd AnaDash/software/driver
cargo build --release
```
This is WIP! I'll make an installer later!

### Debugging
There is an UART port at the main board for debugging!
Also, there are also some key combinations baked in the firmware:
- You can press key1 and key8 (the two outhermost keys) to reset the pico!
- You can press key4 and key5 (the two innermost keys) to go into DFU mode!

## BOM
- 1x Raspberry Pi Pico
- 2x JST-XH 5P Male to Male Cables
- 4x JST-XH 5P Female Vertical Through Hole Sockets
- 2x JST-XH 4P Male to Male Cables
- 5x JST-XH 4P Female Vertical Through Hole Sockets
- 4x 85C1 5mA Ammeters
- 4x 660-680Ω Resistors
- 1x EC11 Rotary Encoder
- 8x Cherry MX Compatible Switches 
- 1x ST7735 1.77 inch LCD TFT Display
- 1x 8 Pin 2.54mm (DuPont) Pin Header 
- 8x Female-to-Male 2.54mm (Dupont) cables
- ?x M3x5x4 Heatset Inserts
- ?x M3 6mm Machine Screws (I used Phillips but I believe Torx could be a better fit if you have the tip)
- 1x Sticker paper
- 1x Case (x 3d printed parts)

PS: The pin header doesn't necessarily need to be male, it can be female, and you can use an male to male cable! Male cables are easier to get in the right amount though... 

## Software used:
This project was designed in:
[KiCad](https://www.kicad.org) (for the PCB)

## TODO List:
- [X] Design the main PCB 
- [X] Design the input PCB
- [X] Design the meters
- [X] Make the drivers
- [ ] Make the driver into an actual driver
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