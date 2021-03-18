use crate::java_ast::{GenJavaCode, JavaFile};
use rustpython_parser::error::ParseError;
use rustpython_parser::{ast, parser};

pub mod java_ast;

pub fn parse_ast(input: &str) -> Result<ast::Program, ParseError> {
    parser::parse_program(input)
}

trait Py2Java {
    type Item: GenJavaCode;

    fn to_java(&self) -> Self::Item;
}

impl Py2Java for ast::Program {
    type Item = JavaFile;

    fn to_java(&self) -> Self::Item {
        unimplemented!()
    }
}

pub fn convert_to_java(prog: &ast::Program) -> String {
    let mut buffer = String::new();
    prog.to_java().gen_java_code(&mut buffer).unwrap();
    buffer
}
