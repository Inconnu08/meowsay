# Meowsay

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

`meowsay` is a rust program that generates ASCII art pictures of a cat with a message. This is a based on the classic `cowsay` program. 
I learnt this watching YouTube tutorials and The Rust CLI Book. 
The program also includes integration tests. This program was built for eductional purpose.

Basic usage:

```
USAGE:
    meowsay.exe [FLAGS] [OPTIONS] [message]

FLAGS:
    -b, --full       Full view
    -h, --help       Prints help information
    -i, --stdin      Read the message from stdin
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    Use your own ASCII cat from file

ARGS:
    <message>    meow? [default: Created by Taufiq Rahman]
```