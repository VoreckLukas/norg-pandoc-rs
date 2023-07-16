use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{exit, Command, Stdio},
    sync::Arc,
};

use clap::Parser;
use walkdir::WalkDir;

/// This is a cli tool to convert a norg tool to any pandoc supported file format.
///
/// It uses pandoc under the hood
#[derive(Parser)]
struct Args {
    /// The file format to convert to
    #[arg(short, long)]
    to: String,

    /// Arguments to pass on to Pandoc
    #[arg(short, long)]
    pandoc: Option<String>,

    /// The output file/directory name
    ///
    /// The default behaviour will place output files with the same name right next to input files
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// The maximum number of threads to use when parsing multiple files
    ///
    /// Defaults to double the number of CPUs
    #[arg(short, long)]
    jobs: Option<usize>,

    /// The input file/directory
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("Input path not found");
        exit(1);
    }

    if args.input.is_file() {
        let output = match args.output {
            Some(mut output) => {
                if output.is_file() {
                    output
                } else {
                    let mut filename = PathBuf::new();
                    filename.push(args.input.file_name().unwrap());
                    filename.set_extension(&args.to);
                    output.push(filename);
                    output
                }
            }
            None => {
                let mut output = args.input.clone();
                output.set_extension(&args.to);
                output
            }
        };
        parse_file(args.input, &args.to, args.pandoc.as_deref(), output);
    } else {
        let output = if let Some(output) = args.output {
            if output.is_file() {
                eprintln!("When the input is a directory, the output can't be a file");
                exit(2);
            }

            output
        } else {
            args.input.clone()
        };
        let directory_walker = WalkDir::new(&args.input).into_iter();
        let thread_pool = if let Some(jobs) = args.jobs {
            rusty_pool::Builder::new()
                .name("norg_pandoc".to_string())
                .core_size(jobs / 2)
                .max_size(jobs)
                .build()
        } else {
            rusty_pool::Builder::new()
                .name("norg_pandoc".to_string())
                .build()
        };
        let to = Arc::new(args.to);
        let pandoc_args = Arc::new(args.pandoc);
        for entry in directory_walker {
            let entry = entry.unwrap().path().to_path_buf();
            if entry.is_file()
                && entry
                    .extension()
                    .map_or(false, |e| e.to_str().map_or(false, |e| e == "norg"))
            {
                let mut output = output.clone();
                output.push(entry.strip_prefix(&args.input).unwrap());
                output.set_extension(&*to);
                let to = to.clone();
                let pandoc_args = pandoc_args.clone();
                thread_pool.execute(move || {
                    parse_file(entry, &*to, pandoc_args.as_deref(), output);
                });
            }
        }
        thread_pool.join();
    }
}

fn parse_file(file: PathBuf, to: &str, pandoc_args: Option<&str>, output_file: PathBuf) {
    if !output_file.parent().unwrap().exists() {
        if let Err(e) = fs::create_dir_all(output_file.parent().unwrap()) {
            eprintln!(
                "Error creating directory {}: {e}",
                output_file.parent().unwrap().to_str().unwrap()
            );
            exit(3);
        }
    }

    let ast = norg_pandoc_ast::parse(file, to);

    let mut pandoc_command = Command::new("pandoc");

    if let Some(arg) = pandoc_args {
        pandoc_command.arg(arg);
    }
    let mut pandoc_command = pandoc_command
        .arg("--from=json")
        .arg("-o")
        .arg(output_file)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn pandoc");
    let mut stdin = pandoc_command.stdin.take().unwrap();
    stdin
        .write_all(ast.to_json().as_bytes())
        .expect("Couldnt write to pandocs stdin");
    stdin.flush().unwrap();
    drop(stdin);
    let _ = pandoc_command.wait();
}
