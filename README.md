<div>

Norg Pandoc Parser {#Norg Pandoc Parser1}
==================

This is a tool for parsing norg files into any other file format with
pandoc. Multiple files will be parsed in a multithreaded fashion.

This README is about the binary you can actually use. If you\'re
interested in current limitations or want to use the library yourself,
please look at [the libraries README](lib/README.md) (TODO This is
probably the more interesting README)

<div>

Dependencies {#Dependencies2}
------------

As this tool uses pandoc, you obviously have to have that installed

</div>

<div>

Usage {#Usage2}
-----

This tool has a bunch of cli Arguments

-   Required:

    -   The input path. If it is given a file it will only parse that
        one. If it is given a directory it will parse any norg file it
        can find

    -   `-t`/`--to` The file format to convert to

-   Optional

    -   `-p`/`--pandoc` The string following this flag will be directly
        passed to pandoc as an argument

    -   `-o`/`--output` Directory/File to save the parsed result to. If
        the input is a directory but this flag is given a file it will
        error out

    -   `-j`/`--jobs` The number of threads to use to parse the
        directory. The default is double the number of available CPUs

</div>

</div>
