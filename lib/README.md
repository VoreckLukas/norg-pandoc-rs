@document\.meta
title: Neorg Pandoc Lib
description: The README of the library backbone
authors: Lukas Voreck
updated: 2023-08-27
version: 1\.0\.0
@end

\* Neorg Pandoc Lib

  This is an attempt at implementing a pandoc reader in rust that can handle the norg File Format. Since 
  pandoc has no native support for rust it will output the AST root node
  Currently I'm working on reimplementing layer 1

\*\* Limitations

\*\*\* Inherited from Tree Sitter

    Since this parser uses the norg tree sitter under the hood it shares the same limitations. 

    I'll List them here as I encounter them.

\*\*\* Inherent to this parser

    The following are limitations that aren't inherited by tree sitter and i have not found a 
    way to resolve yet
