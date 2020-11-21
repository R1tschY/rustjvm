use std::fs::File;

use clap::{App, Arg};

use classfile::descriptor::parse_field_descriptor;
use classfile::parse::parse_class_file;

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
    let constant_pool = class_file.constant_pool();
    for (i, constant) in constant_pool.all() {
        println!("  {:?} => {:?}", i, constant);
    }

    println!(
        "THIS CLASS: {:?} ({:?})",
        class_file.this_class(),
        constant_pool.get(class_file.this_class()).unwrap()
    );
    println!(
        "SUPER CLASS: {:?} ({:?})",
        class_file.super_class(),
        constant_pool.get(class_file.super_class()).unwrap()
    );

    for interface in class_file.interfaces() {
        println!(
            "INTERFACE: {:?} -> {:?}",
            interface,
            constant_pool.resolve_utf8(*interface).unwrap()
        );
    }

    for field in class_file.fields() {
        println!("FIELD:",);
        println!(
            "  NAME: {:?} -> {}",
            field.name_index,
            constant_pool.resolve_utf8(field.name_index).unwrap()
        );
        println!("  FLAGS: {:?}", field.access_flags);
        println!(
            "  DESCRIPTOR: {:?} -> {:?}",
            field.descriptor_index,
            parse_field_descriptor(constant_pool.resolve_utf8(field.descriptor_index).unwrap())
                .unwrap()
        );
        println!("  ATTRIBUTES:");
        for attribute in &field.attributes {
            println!("    {:#?}", attribute);
        }
    }

    for method in class_file.methods() {
        println!("METHOD:",);
        println!(
            "  NAME: {:?} -> {}",
            method.name_index,
            constant_pool.resolve_utf8(method.name_index).unwrap()
        );
        println!("  FLAGS: {:?}", method.access_flags);
        println!(
            "  DESCRIPTOR: {:?} -> {}",
            method.descriptor_index,
            constant_pool.resolve_utf8(method.descriptor_index).unwrap()
        );
        println!("  ATTRIBUTES:");
        for attribute in &method.attributes {
            println!("    {:#?}", attribute);
        }
    }

    println!("ATTRIBUTES:");
    for attribute in class_file.attributes() {
        println!("    {:#?}", attribute);
    }
}
