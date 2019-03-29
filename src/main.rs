extern crate pdf_extract;
extern crate lopdf;

use std::env;
use std::path::PathBuf;
use std::path;
use std::io::BufWriter;
use std::fs::File;
use pdf_extract::*;
use lopdf::*;

fn main() {
    
    let file = env::args().nth(1).unwrap();
    let target = env::args().nth(2).unwrap();
    let collect:Vec<&str> = target.as_str().split(".").collect();
    let filename = collect[0];
    let output_kind = collect[1];
    let path = path::Path::new(&file);
    let mut output_file = PathBuf::new();

    output_file.push(filename);
    output_file.set_extension(&output_kind);
    let mut output_file = BufWriter::new(File::create(output_file).expect("could not create output"));
    let doc = Document::load(path).unwrap();

    print_metadata(&doc);
    let mut output: Box<OutputDev> = Box::new(PlainTextOutput::new(&mut output_file as (&mut std::io::Write)));
    output_doc(&doc, output.as_mut());
}