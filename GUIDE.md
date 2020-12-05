[note] This is a WIP Guide that gets updated while working on the code to document the process.
A full proper wrtie-up will follow.

# TODO CLI - An Introduction to Rust

To get started, download Rust on your computer. To do so please follow the instructions you find on the [getting started](https://www.rust-lang.org/learn/get-started) page of the official rust website.
You will find instructions also to integrate the language with your favorite editor.

Along with the Rust compiler itself, Rust comes with a tool called [Cargo](https://doc.rust-lang.org/cargo/index.html).
Cargo is the Rust package manager, think of it as the “npm” of Rust.

## Start the new project
To start a new project navigate to where you want your project to be created then simply run ` cargo new <project-name>`
In my case I have decided to name my project “*todo-cli*” so I can run
```console
$ cargo new todo-cli
```

Now navigate to the newly created directory and list its content: you should see two files in there
```console
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs
```
Feel free to make any changes to the `Cargo.toml` file as you see fit.

We will work on `src/main.rs` file for the rest of this tutorial, so go ahead and open it.
Like many other languages Rust has a main function that will be run first.
`fn` is how we declare functions while the `!` in `println!` is a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=macro#macros).
As you may guess the program is the standard “*hello world!*”.


To build and run it simply execute  `cargo run`:
```console
$ cargo run
Compiling todo_cli v0.1.0 (/home/claudio/projects/personal/rust/todo_cli)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
    Running `target/debug/todo_cli`
Hello world!
```
as expected.

## First step: Read the arguments

Time to work on our project. Our goal is to have our cli to accept two arguments: the first one which will be the action, and the second one will be the item.

We will start by reading the arguments the user input and printing them out: **replace** the content of main with the following:
```rust
let action = std::env::args().nth(1).expect("Please specify an action");
let item = std::env::args().nth(2).expect("Please specify an item");

println!("{:?}, {:?}", action, item);
```

Let’s start by digesting all this information.




