use classfile::descriptor::parse_field_descriptor;
use classfile::model::ClassFile;
use classfile::parse::parse_class_file;
use std::fs::File;
use std::path::PathBuf;

fn test_resource(resource: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/classes");
    path.push(resource);
    path
}

#[test]
fn private_final_field() {
    let resource = test_resource("EverythingClass.class");

    let mut file = File::open(resource).unwrap();
    let class_file = parse_class_file(&mut file).unwrap();
    objdump(&class_file);
}

fn objdump(class_file: &ClassFile) {
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
        "THIS CLASS: {:?} ({})",
        class_file.this_class(),
        constant_pool
            .resolve_utf8(
                constant_pool
                    .resolve_class(class_file.this_class())
                    .unwrap()
            )
            .unwrap()
    );
    println!(
        "SUPER CLASS: {:?} ({})",
        class_file.super_class(),
        constant_pool
            .resolve_utf8(
                constant_pool
                    .resolve_class(class_file.super_class())
                    .unwrap()
            )
            .unwrap()
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
