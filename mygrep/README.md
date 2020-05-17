## Simple implementation of `grep`

### Command line tool for faster search

This is a simple grep cli tool build with rust.

Currently this allows you to search for a string in a specified file.

Case sensitivity is also supported via an env var **CASE_INSENSITIVE**.

When this var is set, grep with search for any occurrance of the string.

This cli-tool expects 3 args, Usage: **./mygrep <string&gt; &lt;filename>** 
