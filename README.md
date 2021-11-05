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
        --radians         All subsequent angles are specified in radians
        --degrees         All subsequent angles are specified in degrees

TRANSFORMATIONS:
    Tranformations are applied in the order specified, left to right. By
    default, all angles are specified in degrees. If the --radians flag
    appears, all angles after the flag will be interpreted as radians until
    the --degrees flag appears.
	
    --rx <theta>      Rotate model around the X axis by angle theta
    --ry <theta>      Rotate model around the Y axis by angle theta
    --rz <theta>      Rotate model around the Z axis by angle theta
    -t, --translate <X,Y,Z>   Translate model by specified amount
    -mx               Mirror model through the YZ plane
    -my               Mirror model through the XZ plane
    -mz               Mirror model through the XY plane
```
