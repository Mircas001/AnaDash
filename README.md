# AnaDash - An dashboard for your PC
This is an dashboard for your PC that allows you to get a complete breakdown on the PC at an quick glance!

## Features:
- 4x Analog Gauges showing CPU usage, CPU temperature, RAM usage and Swap usage, driven by an Adafruit MCP4728 module!
- 8x Cherry MX keys for any macro your heart wishes!
- An OLED display that can show animations, time, notification and current song!
- An rotary encoder for UI navigation and changing your volume! Including a switch for mute!

## Cad Model
(insert cad here)

## PCB
This project was designed in KiCad, it uses a pair of 2 layer PCBs, the PCBs were split because the switches had to be at an angle while the pico had to sit straight.
For this to work, you must connect the following, all of the cables are JST-XH, as it features an locking mechanism.
You must make the following connections: (Pinouts are in left to right in the mainBoard, GND will always be the rounded rectangle!) (All connections are in the main board)
- DisplayConnector (LED, SCK, SDA, AO, RESET, CS, GND, VCC) - connect this to the matching pins on the display.
- LeftKeyConnector (Switch1, Switch2, Switch3, Switch4, GND) - connect this to the matching connector in the auxiliary board (in the same pinout order when viewed from the back, effectively rendering the same) 
- RightKeyConnector (Switch5, Switch6, Switch7, Switch8, GND) - connect this to the matching connector in the auxiliary board (in the same pinout order when viewed from the back, effectively rendering the same)
- EncoderConnector (B, A, Switch, GND) - connect this to the matching connector in the axuiliary board (in the same pinout order)

### Main PCB
This Board features the Pico, the DAC, the outputs for the ammeters, the display connector and the connections to the input board
[You can check out the PCB and schematic on KiCanvas!](FIXME)
[3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/hardwa/images/mainBoard3Dmodel.png)

### Input PCB
This PCB has all the keys and the encoder and serves to take inputs
[You can check out the PCB and schematic on KiCanvas!](FIXME)
![3D Model](https://github.com/Mircas001/AnaDash/blob/main/hardware/images/inputBoard3Dmodel.png)

## Drivers
You need to install drivers for this, I only support Linux for now, but will gladly give support to anyone making drivers to other systems!

PKGBUILD method:
(insert instruction here FIXME)

Compile it yourself:
(insert instruction here FIXME)

## Firmware
It has been written with MicroPython (should consider changing that)

## BOM
1x Raspberry Pi Pico
2x JST-XH 5P Male to Male Cables
4x JST-XH 5P Female Vertical Through Hole Sockets
2x JST-XH 4P Male to Male Cablex
4x JST-XH 4P Female Vertical Through Hole Sockets
4x 85C1 Ammeter
4x 680Ω Resistors
1x EC11 Rotary Encoder
8x Cherry MX Compatible Switches 
1x ST7735 1.8 inch LCD TFT Display
8x Dupont Socket
8x Female-to-Male Dupont cables
?x M3x5x4 Heatset Inserts
?x M3 6mm Machine Screws (I used Phillips but I believe Torx could be a better fit if you have the tip)
1x Case (x 3d printed parts)

## Software
This project was designed in:
[KiCad](https://www.kicad.org) (for the PCB)

## TODO:
[X] Design the main PCB 
[X] Design the input PCB
[] Make the drivers
[] Make the firmware
[] Make one PCB that is the two others panelized, for money saving
[] Make the Case

## Credits
[Computing: The Details, for giving me the inspiration to start this project a few yeaars ago](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
[Hack Club Macondo program, for the funding:](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
[OrpheusPad, for serving as reference as to how should the git repo and README look like](https://github.com/qcoral/orpheuspad/tree/main)
[HackPad program, which, despite not participating in it, guided me through this](https://hackpad.hackclub.com/)

