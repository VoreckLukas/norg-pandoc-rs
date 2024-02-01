use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    sync::Arc,
};

use clap::{arg, command, Args, Parser};
use walkdir::WalkDir;

/// This is a cli tool to convert a norg file to any pandoc supported file format
///
/// It calls pandoc under the hood, please have it installed
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
    // Allow the user to use a separate path to the pandoc binary through env vars
    let pandoc_path = std::env::var("PANDOC_PATH").unwrap_or("pandoc".to_string());

    // have to construct the cli parser like this, as the derive one
    // doesnt support trailing varargs
    let cli = Arguments::augment_args(command!())
        .arg(arg!([PANDOC_ARGS] ... "arguments to pass on to pandoc").last(true));

    // parse the cli args
    let matches = cli.get_matches();

    // Get the cli options
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

    let api_version = get_api_version(&pandoc_path);

    if input.is_file() {
        // Only process one file

        let output = output.unwrap_or_else(|| {
            let mut output = input.clone();
            output.set_extension(&to);
            output
        });

        parse_file(
            &input,
            pandoc_args.as_deref(),
            &output,
            api_version,
            &pandoc_path,
        );
    } else {
        let output = output.unwrap_or(input.clone());

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

        // Create arcs
        let pandoc_args = Arc::new(pandoc_args);
        let pandoc_path = Arc::new(pandoc_path);

        for entry in directory_walker {
            let entry = entry.unwrap().path().to_path_buf();
            if entry.is_file()
                && entry // Only handle norg files
                    .extension()
                    .map_or(false, |e| e.to_str().map_or(false, |e| e == "norg"))
            {
                // Prepare Arcs etc
                let mut output = output.clone();
                output.push(entry.strip_prefix(&input).unwrap());
                output.set_extension(&*to);
                let pandoc_args = pandoc_args.clone();
                let api_version = api_version.clone();
                let pandoc_path = pandoc_path.clone();

                thread_pool.execute(move || {
                    parse_file(
                        &entry,
                        pandoc_args.as_deref(),
                        &output,
                        api_version,
                        &pandoc_path,
                    )
                });
            }
        }
        thread_pool.join();
    }
}

fn parse_file(
    file: &Path,
    pandoc_args: Option<&str>,
    output_file: &Path,
    api_version: Vec<u32>,
    pandoc_path: &str,
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

    let text = fs::read_to_string(file).expect("Cannot read file");

    let ast = norg_pandoc_ast::parse(&text, api_version);

    let mut pandoc_command = Command::new(pandoc_path);

    if let Some(arg) = pandoc_args {
        pandoc_command.arg(arg);
    }

    let mut pandoc_process = pandoc_command
        .arg("--from=json")
        .arg("-o")
        .arg(output_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn pandoc");

    let mut stdin = pandoc_process.stdin.take().unwrap();
    stdin
        .write_all(ast.to_json().as_bytes())
        .expect("couldn't write to pandoc stdin");
    drop(stdin);

    match pandoc_process.wait_with_output() {
        Ok(output) => println!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
        Err(e) => pandoc_error(e),
    }
}

/// This tool *should* work for all Versions of pandoc, but the generated
/// AST needs to be annotated with a version. I can't hardcode the version
/// So this is a hack to get the API version from the installed pandoc command
fn get_api_version(pandoc_path: &str) -> Vec<u32> {
    let mut pandoc_command = Command::new(pandoc_path)
        .arg("--from=gfm")
        .arg("--to=json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn pandoc");
    let mut stdin = pandoc_command.stdin.take().unwrap();
    stdin.write_all("hack".as_bytes()).unwrap();
    stdin.flush().unwrap();
    drop(stdin);

    match pandoc_command.wait_with_output() {
        Ok(output) => {
            pandoc_ast::Pandoc::from_json(&String::from_utf8_lossy(&output.stdout))
                .pandoc_api_version
        }
        Err(error) => pandoc_error(error),
    }
}

/// Print error and exit if pandoc can*t be called
fn pandoc_error(error: std::io::Error) -> ! {
    eprintln!("Couldn't run pandoc: {error}");
    exit(-1)
}
