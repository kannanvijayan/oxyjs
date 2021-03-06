
extern crate liboxyjs;

use liboxyjs::parser::ast::AstNode;
use liboxyjs::parser::ast_builder::AstBuilder;
use liboxyjs::parser::input_stream::VecInputStream;
use std::io;
use std::io::Read;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;

fn main() {
    liboxyjs::initialize_library();

    let mut stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut buf).unwrap();

    // Create an AstBuilder
    let stream = VecInputStream::new(buf);
    let mut builder = AstBuilder::new(stream);

    let program = builder.parse_program().unwrap();
    println!("Parsed program: {}", program.tree_string())
}
