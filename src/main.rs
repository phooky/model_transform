extern crate stl_io;
extern crate glam;

use std::io::{Write,Read,Seek,stdout,stdin,Cursor};
use std::fs::File;
use glam::{Affine3A,Vec3};
use stl_io::{Vertex,Triangle};

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

fn usage() {
    println!( r#"Simple model affine transformations
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
    -mz               Mirror model through the XY plane"#);
}

fn real_main() -> Result<(),String> {

    let mut transforms : Vec<Affine3A> = Vec::new();
    let mut i = std::env::args().skip(1);
    let mut outpath : Option<String> = None;
    let mut inpath : Option<String> = None;
    
    while let Some(arg) = i.next() {
	match arg.as_str() {
	    "-h" | "--help" => {
		usage();
		return Ok(());
	    },
	    "-o" | "--output" => match i.next() {
		Some(path) => outpath = Some(path),
		None => return Err(format!("{} flag requires a parameter!",arg)),
	    },
	    "-i" | "--input" => match i.next() {
		Some(path) => inpath = Some(path),
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

    trait ReadSeek : Read + Seek {};
    impl<T> ReadSeek for T where T: Read + Seek {};
    
    let mut input = match inpath.clone() {
	Some(x) => Box::new(File::open(x).unwrap()) as Box<dyn ReadSeek>,
	None => {
	    let mut tmpfile = Cursor::new(Vec::new());
	    std::io::copy(&mut stdin(),&mut tmpfile);
	    tmpfile.seek(std::io::SeekFrom::Start(0));
	    Box::new(tmpfile) as Box<dyn ReadSeek>
	},
    };

    // Compile transforms
    let mut final_t = Affine3A::IDENTITY;
    for transform in transforms.iter() {
	final_t = *transform * final_t;
    }

    fn v_to_v3(v : stl_io::Vector<f32>) -> Vec3 {
	Vec3::new(v[0],v[1],v[2])
    }

    fn v3_to_v(v : Vec3) -> stl_io::Vector<f32> {
	stl_io::Vector::new(v.to_array())
    }
    
    match stl_io::create_stl_reader(&mut input) {
	Ok(mut mesh) => {
	    let mut out_tris = Vec::new();
	    for tri in mesh {
		let tri = tri.unwrap();
		let n = v3_to_v(final_t.transform_vector3(v_to_v3(tri.normal)));
		let v0 = v3_to_v(final_t.transform_point3(v_to_v3(tri.vertices[0])));
		let v1 = v3_to_v(final_t.transform_point3(v_to_v3(tri.vertices[1])));
		let v2 = v3_to_v(final_t.transform_point3(v_to_v3(tri.vertices[2])));
		out_tris.push(Triangle { normal : n, vertices : [v0, v1, v2] });
	    }
	    stl_io::write_stl(&mut output, out_tris.iter()).unwrap();
	},
	_ => return Err(format!("Could not read STL file '{:?}'.",inpath)),
    }

    /* We can't easily write indexed meshes via stl_io at the moment *eyeroll* 
    match stl_io::read_stl(&mut input) {
	Ok(mut mesh) => {
	    for mut v in mesh.vertices.iter_mut() {
		let vec_out = final_t.transform_point3(Vec3::new(v[0],v[1],v[2]));
		*v = Vertex::new(vec_out.to_array()); 
		println!("VTX {} {} {}",v[0],v[1],v[2]);
	    }
	    stl_io::write_stl(&mut output, mesh.iter()).unwrap();
	},
	_ => return Err(format!("Could not read STL file '{:?}'.",inpath)),
    }
*/
    Ok(())
}

fn main() {
    match real_main() {
	Ok(()) => (),
	Err(msg) => println!("Error: {}",msg),
    }
}
