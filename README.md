<div>

# Norg Pandoc Parser

This is a tool for parsing norg files into any other file format with
pandoc. Multiple files will be parsed in a multithreaded fashion.

This README is about the binary you can actually use. If you're
interested in the library itself, read it's [README](lib/README.md), if
you're interested in current limitations, read it's [Section about
it](lib/README.md#Limitations2)

<div>

## Feature Requests

This tool obviously doesn't support everything yet. And because I'm the
only one I know of that uses it, I'm mostly adding features I need
myself. If you use it and want to see a specific feature implemented
leave an issue or pull request.

</div>

<div>

## Dependencies

As this tool uses pandoc, you obviously have to have that installed

</div>

<div>

## Usage

This tool has a bunch of cli Arguments

- Required:

  - The input path. If it is given a file it will only parse that one.
    If it is given a directory it will parse any norg file it can find

  - `-t`/`--to` The file format to convert to

- Optional

  - `-o`/`--output` Directory/File to save the parsed result to. If the
    input is a directory but this flag is given a file it will error out

  - `-j`/`--jobs` The number of threads to use to parse the directory.
    The default is double the number of available CPUs

  - All arguments that come after `--` followed by a space will be
    passed on to pandoc

</div>

</div>
