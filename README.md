# Weird Crash Demo

This demo demonstrates a bug in SDL/SDL-TFF/Emscripten when trying to draw both text and rectangles.

## How to build this project

Run `make` in the root folder of this project.

## How to use this project

Navigate to the address indicated by the output.  Open the developer console to see debug and error output.

The controls are as follows:

 - Space: Toggles text rendering
 - Left Arrow: Reduces number of rectangles on the screen
 - Right Arrow: Increases the number of rectangles on the screen

## The reported error

```
[.WebGL-0x135865e00]GL ERROR :GL_INVALID_OPERATION : glDrawArrays: attempt to access out of range vertices in attribute 2
```
