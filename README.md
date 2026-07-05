# AnaDash
See the main PCB:
(insert pcb viewer 1 here) 
See the secondary PCB:

An cool looking dashboard for your PC!
## Features:
- 4x Analog Gauges showing CPU usage, CPU temperature, RAM usage and Swap usage, driven by an Adafruit MCP4728 module.
- 8x Cherry MX keys for any macro your heart wishes.
- An OLED display that can show animations, time, notification and current song. 
- An rotary encoder for UI navigation and changing your volume (including a switch).

## Cad Model:
(insert cad here)

## PCB:
The PCB was split in two, to allow the dashboard to sit at an angle, it has been made fully in KiCad.
This is the main PCB, which takes care of the DAC and the Pico, it also has the display output:
![Schematic](https://github.com/Mircas001/AnaDash/blob/main/pcb/assets/MainBoardSchematic.png)
![PCB](https://github.com/Mircas001/AnaDash/blob/main/pcb/assets/MainBoardPCB.png)
![3D Model](https://raw.githubusercontent.com/Mircas001/AnaDash/refs/heads/main/pcb/assets/MainBoard3DModel.png)

You MUST connect them together, making sure the pins are aligned

This is the secondary/auxiliary board, which has the switches as well the encoder.
(insert schematic and pcb)

## Drivers
This board requires drivers to be installed, I have written them on rust (maybe I should something more well-know or begginer friendly).
It only supports Linux, but I would be glad to help you out if you want to port to other systems.

## Firmware
It has been written with MicroPython (should consider changing that)

## BOM:
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

## Credits:
![OrpheusPad, for serving as reference as to how should the git repo and README look like](https://github.com/qcoral/orpheuspad/tree/main)
![HackPad program, which, despite not participating in it, guided me through this](https://hackpad.hackclub.com/)

