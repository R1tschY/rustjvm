use clap::{App, Arg};
use rustjvm::{parse_class_file, Constant, ConstantIndex};
use std::fs::File;

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

    println!("MAGIC: {:X}", class_file.magic());
    println!(
        "VERSION: {}.{} (Java {})",
        class_file.major_version(),
        class_file.minor_version(),
        class_file.major_version() - 44
    );

    println!("CONSTANTS: {}", class_file.constants().len());
    for (i, constant) in class_file.constants().iter().enumerate() {
        println!("  {:?} => {:?}", ConstantIndex(i as u16), constant)
        /*        match constant {
            Constant::Class { name_index } => {
                println!("  {}: {}", ConstantIndex(i as u16), name_index)
            }
            Constant::Fieldref { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::Methodref { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::InterfaceMethodref { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::String { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::Integer { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::Float { .. } => {}
            Constant::Long { .. } => {}
            Constant::Double { .. } => {}
            Constant::NameAndType { .. } => {}
            Constant::Utf8 { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::MethodHandle { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::MethodType { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
            Constant::InvokeDynamic { .. } => {println!("  {}: {}", ConstantIndex(i as u16), name_index)}
        }*/
    }
}
