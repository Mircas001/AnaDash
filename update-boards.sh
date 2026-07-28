#!/bin/bash
cd hardware
"/usr/bin/kicad-cli" pcb export step --subst-models --include-silkscreen --include-soldermask --user-origin='158.050000x94.900000mm' --min-distance='0.001mm' -f -o "assets/mainBoard.step" "./mainBoard/mainBoard.kicad_pcb"
"/usr/bin/kicad-cli" pcb export step --subst-models --include-silkscreen --include-soldermask --user-origin='158.050000x94.900000mm' --min-distance='0.001mm' -f -o "assets/inputBoard.step" "./inputBoard/inputBoard.kicad_pcb"