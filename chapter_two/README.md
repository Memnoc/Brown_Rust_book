# Chapter Two

This chapter is about programming a guessing game and working with user input, the standard library and strings

## use std::io

This line imports `io` (input/output) set of utils from the standard library `std`
This is all very C like
One interesting thing to notice is that the `stdin` function returns an instance of `std::io:Stdin` which is a type that represents a handle to the standard input for your terminal.
If you read the docs, the signature shows you that precisely: `pub fn stdin() -> Stdin`

## let and mut

We can already see some main components of Rust here, like the use of the keyword `let` for immutable variables.
`mut` is what you need to change the default immutability of Rust variables

## String

This is one of the types provided by the `std` library that is growable and UTF-8 encoded

## read_line(&mut guess)

This is an interesting method, and it basically takes whatever the user types into standard input and append it to a string
The `&` sing means that variable is a **reference** which is a super important concept in Rust (and C as well) - a reference allows you to access data without copying it into memory, thus saving resources and making things faster. Like variables, **references are immutable by default**
