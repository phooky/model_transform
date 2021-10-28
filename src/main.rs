extern crate clap;
extern crate stl_io;
extern crate glam;

use std::io::{Write,stdout};
use std::fs::File;
use clap::{Arg,App};
use glam::{Affine3A,Vec3};

#[derive(Debug,Clone)]
struct ArgParseError;

fn collect_transforms(matches : &clap::ArgMatches, name : &str,
		      f : impl Fn(&str) -> Result<Affine3A,ArgParseError>)
		      -> Result<Vec<(usize,Affine3A)>,ArgParseError> {
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
	     .short("o").long("output")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Name of the file to output the transformed file as, '--' for stdout") )
	.arg(Arg::with_name("rx")
	     .long("rx")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Rotate around the X axis by the specified number of degrees") )
	.arg(Arg::with_name("ry")
	     .long("ry")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Rotate around the Y axis by the specified number of degrees") )
	.arg(Arg::with_name("rz")
	     .long("rz")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Rotate around the Z axis by the specified number of degrees") )
	.arg(Arg::with_name("translate")
	     .short("t").long("translate")
	     .multiple(true)
	     .number_of_values(1)
	     .takes_value(true)
	     .help("Translate by the given X,Y,Z triple") )
	.arg(Arg::with_name("mx")
	     .long("mx")
	     .multiple(true)
	     .help("Mirror the model across the YZ plane") )
	.arg(Arg::with_name("my")
	     .long("my")
	     .multiple(true)
	     .help("Mirror the model across the XZ plane") )
	.arg(Arg::with_name("mz")
	     .long("mz")
	     .multiple(true)
	     .help("Mirror the model across the XY plane") );

    let matches = app.get_matches();
    
    let mut output = match matches.value_of("output") {
	None => Box::new(stdout()) as Box<dyn Write>,
	Some(x) => Box::new(File::create(x).unwrap()) as Box<dyn Write>,
    };
    let mut transformations : Vec<(usize,Affine3A)> = Vec::new();
    fn make_translation(arg : &str) -> Result<Affine3A,ArgParseError> {
	let args : Vec<&str> = arg.split(",").collect();
	if args.len() != 3 {
	    return Err(ArgParseError);
	}
	let mut coords : [f32; 3] = [0.0; 3];
	for (i,v) in args.iter().enumerate() {
	    coords[i] = match v.parse::<f32>() {
		Ok(val) => val,
		Err(_) => return Err(ArgParseError),
	    }
	}
	println!("Parsed: {} {} {} ",coords[0],coords[1],coords[2]);
	let v = Vec3::new(coords[0],coords[1],coords[2]);
	Ok(Affine3A::from_translation(v))
    }
    collect_transforms(&matches, "translate", make_translation );
		    
}
