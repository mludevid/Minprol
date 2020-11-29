# MINPROL

Minprol (Minimalistic Programming Language) is an interpreted programming language with the goal of being as minimalistic as possible without losing any functionallity.
We'll see how that goes.

A documentation of the language is not planned until it has reached a more mature status.

At the moment it is a simple math expressions solver with functionallity for variables, blocks, functions and it starts to have support for booleans and if-else-statements.
It can handle `+`, `-`, `*`, `/` and `()` for I32 (`1`), I64 (`1l`), F32 (`1.0`), F64 (`1.0d`), U32 (`1u`) and U64 (`1ul`) without mixing them.

To run it you'll need the Rust compiler. You can start the interpreter by running `cargo run`.

Next steps:
- Work in progress: Write test functions for everything.
- Improve end of line handling to later be able to accept instructions over multiple lines in files.
- If Else
- Loops
- Reform data types and add new ones like Arrays.
- Improve error handeling

Little improvements:
- Try to move asigning values into math_value to be able to delete process_expression
- Improve efficiency of blocks
- Check that function inputs are Identifiers on declaration so it panickes then instead of when calling the funciton.
