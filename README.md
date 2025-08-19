# custom-ddp-controller
A simple rust application for controlling LEDs via the DDP protocol

## To Do
- [x] Make timing work for longer running routines (seconds converted to floats gets less accurate with longer times)
    - See `demos::rainbow_oscillation` for the correct solution, which uses modulo on unsigned integers
    - [ ] Would be useful as a struct with implementation