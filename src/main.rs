use std::{env, fs, path::PathBuf};

use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, PandocOutput};
use pandoc_ast::Pandoc;

fn main() {
    let file = env::args().nth(1).unwrap();
    let file = fs::read_to_string(file).unwrap();
    let pandoc = norg_pandoc_convert::parse(&file, get_version());
    let json = pandoc.unwrap().unwrap().to_json();
    let mut pandoc = pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(json))
        .set_input_format(InputFormat::Json, Vec::new())
        .set_output(OutputKind::File(PathBuf::from("test/test.pdf")));
    pandoc.execute().unwrap();
}

fn get_version() -> Vec<u32> {
    let mut pandoc = pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(String::from("t")))
        .set_input_format(InputFormat::Markdown, Vec::new())
        .set_output(pandoc::OutputKind::Pipe)
        .set_output_format(OutputFormat::Json, Vec::new());
    if let PandocOutput::ToBuffer(pandoc) = pandoc.execute().unwrap() {
        Pandoc::from_json(&pandoc).pandoc_api_version
    } else {
        unreachable!()
    }
}
