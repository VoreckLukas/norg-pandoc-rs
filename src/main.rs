use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    sync::Arc,
};

use clap::{arg, command, Args, Parser};
use walkdir::WalkDir;

/// This is a cli tool to convert a norg tool to any pandoc supported file format.
///
/// It uses pandoc under the hood
#[derive(Parser)]
struct Arguments {
    /// The file format to convert to
    #[arg(short, long)]
    to: String,

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
    let cli = command!();
    let cli = Arguments::augment_args(cli)
        .arg(arg!([PANDOC_ARGS] ... "arguments to pass on to pandonc").last(true));
    let matches = cli.get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap().to_owned();
    let output = matches.get_one::<PathBuf>("output").map(|v| v.to_owned());
    let to = matches.get_one::<String>("to").unwrap().to_owned();
    let pandoc_args: Option<String> = matches
        .get_many::<String>("PANDOC_ARGS")
        .map(|v| v.map(|v| v.to_owned()).collect::<Vec<String>>().join(" "));
    let jobs = matches.get_one::<usize>("jobs").map(|v| v.to_owned());

    if !input.exists() {
        eprintln!("Input path not found");
        exit(1);
    }

    let api_version = get_api_version();

    if input.is_file() {
        let output = match output {
            Some(mut output) => {
                if output.is_file() {
                    output
                } else {
                    let mut filename = PathBuf::new();
                    filename.push(input.file_name().unwrap());
                    filename.set_extension(&to);
                    output.push(filename);
                    output
                }
            }
            None => {
                let mut output = input.clone();
                output.set_extension(&to);
                output
            }
        };
        parse_file(
            &input,
            &to,
            pandoc_args.as_deref(),
            &output,
            api_version,
            output.parent().unwrap(),
        );
    } else {
        let output = if let Some(output) = output {
            if output.is_file() {
                eprintln!("When the input is a directory, the output can't be a file");
                exit(2);
            }

            output
        } else {
            input.clone()
        };
        let workspace_root = Arc::new(input.clone());
        let directory_walker = WalkDir::new(&input).into_iter();
        let thread_pool = if let Some(jobs) = jobs {
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
        let to = Arc::new(to);
        let pandoc_args = Arc::new(pandoc_args);
        for entry in directory_walker {
            let entry = entry.unwrap().path().to_path_buf();
            if entry.is_file()
                && entry
                    .extension()
                    .map_or(false, |e| e.to_str().map_or(false, |e| e == "norg"))
            {
                let mut output = output.clone();
                output.push(entry.strip_prefix(&input).unwrap());
                output.set_extension(&*to);
                let to = to.clone();
                let pandoc_args = pandoc_args.clone();
                let api_version = api_version.clone();
                let workspace_root = workspace_root.clone();
                thread_pool.execute(move || {
                    parse_file(
                        &entry,
                        &to,
                        pandoc_args.as_deref(),
                        &output,
                        api_version,
                        &workspace_root,
                    );
                });
            }
        }
        thread_pool.join();
    }
}

fn parse_file(
    file: &Path,
    to: &str,
    pandoc_args: Option<&str>,
    output_file: &Path,
    api_version: Vec<u32>,
    workspace_root: &Path,
) {
    if !output_file.parent().unwrap().exists() {
        if let Err(e) = fs::create_dir_all(output_file.parent().unwrap()) {
            eprintln!(
                "Error creating directory {}: {e}",
                output_file.parent().unwrap().to_str().unwrap()
            );
            exit(3);
        }
    }

    let ast = norg_pandoc_ast::parse(file, to, api_version, workspace_root);

    let mut pandoc_command = Command::new("pandoc");

    if let Some(arg) = pandoc_args {
        pandoc_command.arg(arg);
    }
    let mut pandoc_command = pandoc_command
        .arg("--from=json")
        .arg("-o")
        .arg(output_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn pandoc");
    let mut stdin = pandoc_command.stdin.take().unwrap();
    stdin
        .write_all(ast.to_json().as_bytes())
        .expect("Couldnt write to pandocs stdin");
    stdin.flush().unwrap();
    drop(stdin);
    match pandoc_command.wait_with_output() {
        Ok(output) => {
            print!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        }
        Err(error) => eprintln!("Couldn't run pandoc: {error}"),
    }
}

fn get_api_version() -> Vec<u32> {
    let mut pandoc_command = Command::new("pandoc")
        .arg("--from=gfm")
        .arg("--to=json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn pandoc");
    let mut stdin = pandoc_command.stdin.take().unwrap();
    stdin
        .write_all("t".as_bytes())
        .expect("Couldnt write to pandocs stdin");
    stdin.flush().unwrap();
    drop(stdin);
    match pandoc_command.wait_with_output() {
        Ok(output) => {
            pandoc_ast::Pandoc::from_json(&String::from_utf8_lossy(&output.stdout))
                .pandoc_api_version
        }
        Err(error) => {
            eprintln!("Couldn't run pandoc: {error}");
            exit(-1);
        }
    }
}
