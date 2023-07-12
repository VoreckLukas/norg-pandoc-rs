<div>

# Pandoc Reader for Norg Files

This is an attempt at implementing a pandoc reader in rust that can
handle the norg File Format. Since pandoc has no native support for rust
it will output the AST in json representation which can then be piped
into pandoc. Example
`./norg-pandoc-tree-sitter README.norg | pandoc --from=json -o README.md`

<div>

## Limitations

<div>

### Inherited from Tree Sitter

Since this parser uses the norg tree sitter under the hood it shares the
same limitations.

I'll List them here as I encounter them.

<div>

#### Headings and Lists

- The max nesting Headings and lists can have is 6. Anything beyond that
  will be treated like 6th level nesting

</div>

</div>

<div>

### Inherent to this parser

The following are limitations that aren't inherited by tree sitter and i
have not found a way to resolve yet

<div>

#### Links

- Line number links (`{2}`) don't work

- Magic Char links to other files (`{:other_file:# Some Heading}`) don't
  work and will simply link to the other file

- File links of the syntax `{file://path/to/file.norg}` seem to work as
  apparantly URIs of that schema need absolute paths. However even with
  absolute paths im running into issues

- Scoping Links of the form `{* Level 1 Heading: *** Level 3 Heading}`
  will simply link to `* Level 1 Heading` As resolving the scope is over
  my head currently

</div>

</div>

</div>

</div>
