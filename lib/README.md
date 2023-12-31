<div>

# Neorg Pandoc Lib

This is an attempt at implementing a pandoc reader in rust that can
handle the norg File Format. Since pandoc has no native support for rust
it will output the AST root node Currently it supports Layer 2 as well
as selected other things:

- Inline link targets

- Wiki Links

- `.image`

- Todo items:

  - Urgent (`(!)`)

<div>

## Limitations

<div>

### Inherited from Tree Sitter

Since this parser uses the norg tree sitter under the hood it shares the
same limitations.

I'll List them here as I encounter them.

<div>

#### Headings, Lists and Quotes

- The max nesting Headings and lists can have is 6. Anything beyond that
  will be treated like 6th level nesting

</div>

<div>

#### Delimiting Modifiers

- The spec says that the delimiting modifiers consist of two or more
  consecutive corresponding chars. However tree sitter only recognizes
  them if the char is repeated three or more times.

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

- Wiki Links currently only work in the same file. As this parser is
  only processing one file at a time it can't open other files to search
  for the target.

- Scoping Links of the form `{* Level 1 Heading: *** Level 3 Heading}`
  will simply link to `* Level 1 Heading` As resolving the scope is over
  my head currently

</div>

<div>

#### Ranged Verbatim Tags

- These need better documentation as to what kinds of tags exist and how
  to handle them. The ones implemented for now are:

  - Metadata `@document.meta`

  - Code blocks

  - Everything else is handled pretty much like a code block

<div>

##### Metadata

- The `@document.meta` tags will always just generate Metadata Strings,
  never lists as there is no syntax for those. This means that if for
  example you define a list of keywords, it will result in just a String
  with your raw list. The spec definitely needs to document these
  better.

</div>

</div>

</div>

</div>

</div>
