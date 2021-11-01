# model_transform
A command-line tool for performing simple affine transformations on STL models.

## Build instructions
1. Install the rust toolchain, if you don't already have it.
2. `cargo build --release`

## Usage
```
Simple model affine transformations
Apply a sequence of simple transformations to an STL model.

USAGE:
    model_transform [-i INPUT] [TRANSFORMATIONS] [-o OUTPUT]

FLAGS:
    -h, --help     Prints help information

OPTIONS:
    -i, --input <INPUT>   Path to input STL file; defaults to stdin
    -o, --output <OUTPUT> Path to output STL file; defaults to stdout

TRANSFORMATIONS:
    Tranformations are applied in the order specified, left to right.
    --rx <theta>      Rotate model around the X axis by theta radians
    --ry <theta>      Rotate model around the Y axis by theta radians
    --rz <theta>      Rotate model around the Z axis by theta radians
    -t, --translate <X,Y,Z>   Translate model by specified amount
    -mx               Mirror model through the YZ plane
    -my               Mirror model through the XZ plane
    -mz               Mirror model through the XY plane
```
