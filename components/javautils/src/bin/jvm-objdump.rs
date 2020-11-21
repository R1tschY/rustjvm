use std::fs::File;

use clap::{App, Arg};

use classfile::model::ConstantIndex;
use classfile::parse_class_file;

fn main() {
    let matches = App::new("JVM objdump")
        .version("0.1")
        .author("Richard Liebscher <richard.liebscher@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let mut file = File::open(input).unwrap();
    let class_file = parse_class_file(&mut file).unwrap();

    println!(
        "VERSION: {}.{} (Java {})",
        class_file.major_version(),
        class_file.minor_version(),
        class_file.major_version() - 44
    );
    println!("FLAGS: {:?}", class_file.access_flags());

    println!("CONSTANTS:");
    for (i, constant) in class_file.constants().iter().enumerate() {
        println!("  {:?} => {:?}", ConstantIndex(i as u16 + 1), constant);
    }

    println!(
        "THIS CLASS: {:?} ({:?})",
        class_file.this_class(),
        class_file
            .resolve_constant(class_file.this_class())
            .unwrap()
    );
    println!(
        "SUPER CLASS: {:?} ({:?})",
        class_file.super_class(),
        class_file
            .resolve_constant(class_file.super_class())
            .unwrap()
    );
}
