# Chapter One

## How to update Rust

You have the command `rustup` to handle the rust versions
If you use `rutstc` you can check which version of Rust you have installed locally

## main()

It is a special function and always the first code that runs in every executable Rust program

## How to format Rust

This is done by default (as it's installed in the Rust tool chain) with `rustfmt`

## Compiling and Running: separate steps

Compiling and running are two different things.
Traditionally, you can compile a Rust program by running `rustc main.rs`. This is very similar to `gcc file.c` in C

If you compile like that, an executable will be created in `/src/` named after the file. So for `main.rs` would be simply `main`

To run the executable, you simply `./main` in the directory where the file is contained

## Hello, Cargo!

So cargo is what you want to use to do everything. Is like Node for JavaScript, so it helps running, compiling, generating project templates and other very useful stuff.
