# AnaDash - An dashboard for your PC
This is an dashboard for your PC that allows you to get a complete breakdown on the PC at an quick glance. 

## Features:
- 4x Analog Gauges showing CPU usage, CPU temperature, RAM usage and Swap usage, driven by an Adafruit MCP4728 module.
- 8x Cherry MX keys for any macro your heart wishes.
- An OLED display that can show animations, time, notification and current song. 
- An rotary encoder for UI navigation and changing your volume (including a switch).

## Cad Model
(insert cad here)

## PCB
This project uses two 2 layer PCBs, it was designed in KiCa

### Main PCB
![Check out an interactive version of it here!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fpcb%2FanadashMainBoard)
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/pcb/assets/mainBoard3Dmodel.png)

### Auxiliary PCB
![Check out the schematic and PCB on the KiCanvas link!](https://kicanvas.org/?repo=https%3A%2F%2Fgithub.com%2FMircas001%2FAnaDash%2Ftree%2Fmain%2Fpcb%2FanadashSecondaryBoard)
![3D Model](https://github.com/Mircas001/AnaDash/blob/main/pcb/assets/auxBoard3Dmodel.png)

## Drivers
You need to install drivers for this, it only supports Linux but I will gladly provide support for anyone who tries making drivers to other systems!
(insert instruction here)

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
1x I2C SSD1306 0.96inch 128x64 White OLED Display
?x M3x5x4 Heatset Inserts
?x M3 6mm Phillip Screws 
1x Case (x 3d printed parts)

## Software
This project was designed in:
![KiCad](https://www.kicad.org) (for the PCB)
![Onshape](https://onshape.com) (for the 3D Model)

## Credits
![Computing: The Details, for giving me the inspiration to start this project a few yeaars ago](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
![Hack Club Macondo program, for the funding:](https://www.youtube.com/watch?v=4J-DTbZlJ5I)
![OrpheusPad, for serving as reference as to how should the git repo and README look like](https://github.com/qcoral/orpheuspad/tree/main)
![HackPad program, which, despite not participating in it, guided me through this](https://hackpad.hackclub.com/)

