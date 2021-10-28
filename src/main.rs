extern crate stl_io;
extern crate glam;

use std::io::{Write,stdout};
use std::fs::File;
use glam::{Affine3A,Vec3};

#[derive(Debug,Clone)]
struct ArgParseError;

fn parse_vec3(argument : &str) -> Result<Vec3,ArgParseError> {
    let args : Vec<&str> = argument.split(",").collect();
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
    Ok(Vec3::new(coords[0],coords[1],coords[2]))
}


fn real_main() -> Result<(),String> {

    let mut transforms : Vec<Affine3A> = Vec::new();
    let mut i = std::env::args().skip(1);
    let mut outpath : Option<String> = None;
    
    while let Some(arg) = i.next() {
	match arg.as_str() {
	    "-o" | "--output" => match i.next() {
		Some(path) => outpath = Some(path),
		None => return Err(format!("{} flag requires a parameter!",arg)),
	    },
	    "-t" | "--translate" => match i.next() {
		Some(spec) => match parse_vec3(spec.as_str()) {
		    Ok(vec) => transforms.push(Affine3A::from_translation(vec)),
		    Err(_) => return Err(format!("Could not parse {} as a triple of floats.",spec)),
		},
		None => return Err(format!("{} requires an argument.",arg)), },
	    "--rx" | "--ry" | "--rz" => match i.next() {
		Some(spec) => match spec.parse::<f32>() {
		    Ok(val) => transforms.push( match arg.as_str() {
			"--rx" => Affine3A::from_rotation_x(val),
			"--ry" => Affine3A::from_rotation_y(val),
			"--rz" => Affine3A::from_rotation_z(val),
			_ => panic!("Unreachable code."),
		    } ),
		    Err(_) => return Err(format!("Could not parse {} as a float.",spec)),
		},
		None => return Err(format!("{} requires an argument.",arg)), },
	    "--mx" => transforms.push( Affine3A::from_scale(Vec3::new(-1.0,1.0,1.0)) ),
	    "--my" => transforms.push( Affine3A::from_scale(Vec3::new(1.0,-1.0,1.0)) ),
	    "--mz" => transforms.push( Affine3A::from_scale(Vec3::new(1.0,1.0,-1.0)) ),
	    _ => return Err(format!("Unrecognized argument {}.",arg)),
	}
    }
    let mut output = match outpath {
	None => Box::new(stdout()) as Box<dyn Write>,
	Some(x) => Box::new(File::create(x).unwrap()) as Box<dyn Write>,
    };
    Ok(())
}

fn main() {
    match real_main() {
	Ok(()) => (),
	Err(msg) => println!("Error: {}",msg),
    }
}

/*
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
    }
    collect_transforms(&matches, "translate", make_translation );
*/		    
