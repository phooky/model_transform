extern crate clap;
extern crate stl_io;
extern crate glam;

use std::io::{Write,stdout};
use std::fs::File;
use clap::{Arg,App};
use glam::{Mat4,Vec4};

#[derive(Debug,Clone)]
struct ArgParseError;

fn collect_transforms(matches : &clap::ArgMatches, name : &str,
		      f : impl Fn(&str) -> Result<Mat4,ArgParseError>)
		      -> Result<Vec<(usize,Mat4)>,ArgParseError> {
    let is = matches.indices_of(name);
    let vs = matches.values_of(name);
    let mut mats = Vec::new();
    match (is,vs) {
	(Some(indices),Some(values)) => {
	    for (idx,val) in indices.zip(values) {
		println!("CT IDX {:?} VAL {:?}",idx,val);
		match f(val) {
		    Ok(m) => mats.push((idx,m)),
		    Err(e) => return Err(e),
		}
	    };
	},
	_ => (),
    };
    Ok(mats)
}

fn main() {
    let app = App::new("Transform a 3d model")
	.about("Perform a set of simple affine transformations on a 3d model.")
	.arg(Arg::with_name("center")
	     .help("Compute the center of the model's bounding box and move it to the origin.") )
	.arg(Arg::with_name("output")
	     .short("o")
	     .takes_value(true)
	     .help("Name of the file to output the transformed file as, '--' for stdout") )
	.arg(Arg::with_name("rx")
	     .takes_value(true)
	     .help("Rotate around the X axis by the specified number of degrees") )
	.arg(Arg::with_name("ry")
	     .takes_value(true)
	     .help("Rotate around the Y axis by the specified number of degrees") )
	.arg(Arg::with_name("rz")
	     .takes_value(true)
	     .help("Rotate around the Z axis by the specified number of degrees") )
	.arg(Arg::with_name("translate")
	     .short("t")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Translate by the given X,Y,Z triple") )
	.arg(Arg::with_name("mx")
	     .help("Mirror the model across the YZ plane") )
	.arg(Arg::with_name("my")
	     .help("Mirror the model across the XZ plane") )
	.arg(Arg::with_name("mz")
	     .help("Mirror the model across the XY plane") );

    let matches = app.get_matches();
    
    let mut output = match matches.value_of("output") {
	None => Box::new(stdout()) as Box<dyn Write>,
	Some(x) => Box::new(File::create(x).unwrap()) as Box<dyn Write>,
    };
    let mut transformations : Vec<(usize,Mat4)> = Vec::new();
    fn make_translation(arg : &str) -> Result<Mat4,ArgParseError> {
	let args : Vec<&str> = arg.split(",").collect();
	if args.len() != 3 {
	    return Err(ArgParseError);
	}
	Ok(Mat4::IDENTITY)
    }
    collect_transforms(&matches, "translate", make_translation );
		    
}
