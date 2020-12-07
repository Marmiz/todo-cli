[note] This is a WIP Guide that gets updated while working on the code to document the process.
A full proper wrtie-up will follow.

# TODO CLI - An Introduction to Rust

To get started, download Rust on your computer. To do so please follow the instructions you find on the [getting started](https://www.rust-lang.org/learn/get-started) page of the official rust website.
You will find instructions also to integrate the language with your favorite editor.

Along with the Rust compiler itself, Rust comes with a tool called [Cargo](https://doc.rust-lang.org/cargo/index.html).
Cargo is the Rust package manager, think of it as the “npm” of Rust.

Being a TODO app that will just run in the terminal, I have decided to store my values as a collection of items and a boolean value representing its active state.

Or visually:
```md
<item> <active: true|false>

"write rust" true
"make coffee" false
"complete all freeCodeCamp" true
```
Means that we have two todos active ("write rust" and "complete all freeCodeCamp"), and a completed one: "make coffee"

## Before we start: general tips.

### Type System
Rust is a strongly typed language, meaning that we will have to take care of variables types, but it comes with a pretty advanced type interference so in many places the complire will "understead" the types sparing us the duty of manually typing them each time.

### Semicolons
Also opposed to Javascript there's no [AFI](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Automatic_semicolon_insertion), meaning that we *have to* type ";" ourselves.

The only exeption is the last statement of a fnction: if no `;` is found, Rust will treat that as a `return` statement.

So if you will have some weird errors while coding along, remeber to check the `;`.


## Start a new project
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

## 1: Read the arguments

Time to work on our project. Our goal is to have our cli to accept two arguments: the first one which will be the action, and the second one will be the item.

We will start by reading the arguments the user input and printing them out. 
**Replace** the content of main with the following:
```rust
let action = std::env::args().nth(1).expect("Please specify an action");
let item = std::env::args().nth(2).expect("Please specify an item");

println!("{:?}, {:?}", action, item);
```

Let’s start by digesting all this information.

- `let` [[doc]](https://doc.rust-lang.org/std/keyword.let.html) binds a value to a variable.
- `std::env::args()` [[doc]](https://doc.rust-lang.org/std/env/fn.args.html) is a function brought in from the *env* module of the standard libray that returns the arguments that the programs was started with.Being an iterator we can access the value stored at each position with the `nth()` function. We start from the 1st position as in Rust arguments parsing, the element at position 0 is the program itself, which we don't really care for right now.

> The Argument at position 0 is the program itself, that's why we start reading from the 1st element.

- `expect()` [[doc]](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect) is a method defined for the `Option` enum that will either return the value, or if not present will terminate the program immediatly (Panic in Rust terms) returning the provided message.

But what's exactly is the `Option` enum that is returned, and why is returned?
Rust does not have *null* or *undefined* values like javascript. This means that Rust enforces us to deal with the case of the value "not being there".

Because the program can be run without arguments, Rust enforces us to check wheter a value is actually provided by giving us an Option type: either the value is there, or not, and we as the programmer have the responsabilty of ensuring that an appropriate action is made on each case.

For the time being if the argument is not provided we will exit the program immediately.
Let's run the program and pass two arguments, to do so append them after `--`. For example:
```console
$ cargo run -- hello world!
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/todo_cli hello 'world'\!''`
"hello", "world!"
```

## 2. Inserting and saving data with a custom type

Let's think for a moment about our goal for the program.
We want to read the argument given by the user, update our todo list and store it somehwere for usage.

To do so, we will implement our own type where we can define our methods to meets the business needs.
We will use Rust's [struct](https://doc.rust-lang.org/std/keyword.struct.html), which let us do both in a clean way avoiding us to write all the code inside the main function. 

### Defining our struct:
Since we will type HashMap a lot we can bring it in into scope and save us some typings.
At the top of our file add this line:
```rust
use std::collections::HashMap
```
This will let us using HashMap directly without the need of typing the full path.


Below the main function let's add the following code:
```rust
struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}
```
This will define our custom Todo type: a struct with a `map` field that is an [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
We can think of HashMaps as a *kind* of Javascript object, but Rust enforces us to declare also the type of the key and value.

When we declared `HashMap<String, bool>` we have told Rust that our HashMap will have keys composed by Strings, the "todo item", and a boolean: the active state. (refear to the introduction to refresh why we choose to do so.)

### Adding methods:
Methods are like regular functions, they are delcared with the `fn` keyword, accept parameters and have a return value. However they differ from regular function in that are defined  within the context of a struct and their first parameters is *always* `self`.

### Inserting into our map
We are gonna define an *impl* (implementation) block below the newly added struct.
```rust
impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // active state is set to true by default.
        self.map.insert(key, true);
    }
}
```
This function is pretty straight-forward: it's simply taking a reference to the struct and a key, and insterting it into our map.

Two very important piece of information here that all lies in this line:
```rust
&mut self
```
 - **mut** [[doc]](https://doc.rust-lang.org/std/keyword.mut.html) makes a variable mutable. In Rust every variable is *immutable* by default, if you want to update a value you need to opt-in mutability using the `mut` keywords. Since with our function we are effectively changing our map by adding a new value, we need it to be declare as mutable.

 - **&** [[doc]](https://doc.rust-lang.org/std/primitive.reference.html) indicates a *reference*. You can imagine the varaible to be a pointer to the memory location where the value is stored, rather the being the "value" itself.<br/>
    In Rust terms this is refered as a **borrow**, meaning that the function is not actually owning this value, but merely pointing to the location where it's stored. 

---
## A brief overview of Rust ownership system
Ownership is Rust's most unique feature. At its core is what enables Rust programmer to write program without the need of manually allocate memory (like in C/Cpp for example) while still be able to run without a Garbage Collector (like JavaScript or Python) that constantly look at our program's memory to free resources not in use.

The onwenrship system has three rules:
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Rust check this rules at compile time, meaning that the programmer has the duty to be explicit if and when you want a value to be freed in memory.

Think of this example:
```rust
fn main() {
 // the owner of the String is x 
 let x = String::from("Hello");
 
 // we move the value inside this function, so now doSomething is the owner of x.
 // Rust will free the memory associated with x as soon as it goes out of "doSomething" scope.
 doSomething(x);

 // The compiler will throw an error since we tried to use the value x
 // but since we moved it inside "doSomething"
 // we cannot use it as we don't have a guarantee that the value have not been dropped.
 println!("{}", x);
}
```

This concept is widely regarded as the hardest to grasp about Rust as it's a concept that may be new to many programmers.
You can read more in-depth explanation about the [Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) from Rust official book.

---

Now that we saw what borrowing means, you should understeand why whe declared `&mut self`: we don't want to drop the map yet, we still need its data to be stored somewhere.

### Saving our map to disk
Now that we have an insert method, we should save the value on a file written to disk so they can be permanently stored. To do so we can define a new method for our struct that will take care of writing the map into a file.

```rust
impl Todo {
    // [rest of the code]

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }

        std::fs::write("db.txt", content)
    }
}
```
Here we have deined a new method `save` that returns a [Result](https://doc.rust-lang.org/std/result/enum.Result.html). We iterate over the map, format each string separating key and value with a tab charachter and each line with a new line.
Then we write it to disk in a file called `db.txt`

It's important to notice that `save` *take ownership* of self. This is an arbitrary decision so the compiler will stop us if we were to accidentally try to update the map after we call `save` as the memory of self will be freed after save goes out of scope.

### Using struct in main

Now that we have this two methods, we can put them at use.
We left off main from the point where we read the argiments supplied. Now if the action supplied is "add" we will insert that item into the file and store it for later use.

Add these lines below the two arguments bindings:
```rust
fn main() {
    // ...[arguments bindig code]

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } 
}
```
Let's see what we are doing here:

- `let mut todo = Todo` is the syntax that let us instantiate a struct and we are binding it as mutable since we have to insert into it, thus performing a mutation.

- we call the `insert` method using the familiar `.` notation.

- we match [[doc]](https://doc.rust-lang.org/std/keyword.match.html) the Result returned from the save function and print a message on screnn for the user.

Let's test it.
Navigate to your terminal and type:
```console
$ cargo run -- add "code rust"
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/todo_cli add 'code rust'`
todo saved
```
Let's inspect the saved item:
```console
$ cat db.txt             
code rust   true  
```
Seems like the item has been added.
Let's try one more time:
```console
$ cargo run -- add "new item"
todo saved
$ cat db.txt
new item    true
```

What happened to the previous item?

## 3. Reading our Database first

Right now our program has a fundamental flaw: each tume we "add" we are overwriting the map instead of updating it. The reason for it is that we create a new empty map every time we run the program.
Time to fix it.

### A new(*pun intended*) function in TODO

We are gonna implement a new function for our Todo struct, that once called will read the content of our file and give us back our Todo with the value previously stored.
Note that this is not a method as it's not taking `self` as first argument.

I am gonna call it `new`, which is just a Rust convention (see `HashMap::new()` as used before), note that new is not a special keyword like in Javascript, so we could have potentially called this function as we liked.

Let's add the following conde inside our `impl` block:
```rust

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let content = std::fs::read_to_string("db.txt")?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    // ...rest of the methods
}
```
No worries if this feels a bit overwhelming, we choose to use a more Functional Programming style for this one, mainly to showcase and itroduce the fact that Rust supporst many features found in other languages such as iterators, closure, and lamda functions.

Let's see what is happening here:

- We are defining a `new` function that will return a Result that is either a `Todo` struct or an `io:Error`.

- We are reading the content of the `db.txt` file into a string.

- `let map: HashMap<String, bool>`: we are binding to the map variable an `HashMap`, here is one of the occasion where the compiler have trouble interfering the type for us, so we declare it ourself.

- `lines` [[doc]](https://doc.rust-lang.org/std/primitive.str.html#method.lines) creates an Iterator over each line of a string, meaning that now we will iterate on each entry of our database, since we are separating them with a `/n` charachter.

- `map` [[doc]](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) takes a closure and calls it on each element of the iterator.

- `line.splitn(2, '\t')` [[doc]](https://doc.rust-lang.org/std/primitive.str.html#method.splitn) will split our lines at most two times on the tab charachter.

- `collect()` [[doc]](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect) as described in the documentation is one of the most powerful method in the standard library. Is a to transform an iterator into a relevant collection. So here we are telling the map function to transform our Splitted stirng into a Vector of borrowed string slices by appending `::Vec<&str>` to the method.

- `.map(|v| (v[0], v[1]))` then we transform it into a touple for convenience.

- `.map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))` then we convert the two elements of the tuple into a `String` using `String::from` and into a `bool` using `bool::from_str`.

- We collect them into our HashMap. Rust knows what to do since we declared explicitly the type we want to have when we made our bindig.

- Lastly if we never encoutered any error we return our struct to the caller with `Ok(Todo { map })`. Note that much like in Javascript we can use a shorter notation if the key and the variable has the same name.

*phew! Not so bad*

---
### An alternative approach.
The above could have also been implemented with a `for` loop instead:
```rust
fn new() -> Result<Todo, std::io::Error> {
    let content = std::fs::read_to_string("db.txt")?;
    let mut map = HashMap::new();

    for entries in content.lines() {
        let mut values = entries.split('\t');
        let key = values.next().expect("No Key");
        let val = values.next().expect("No Value");

        map.insert(String::from(key), bool::from_str(val).unwrap());

    }
    Ok(Todo { map })
}
```
---

### Using the new function

Inside main simply update the binging to our todo variable with:
```rust
/*
remove 
    let mut todo = Todo {
        map: HashMap::new(),
    };
in favor of:
*/
let mut todo = Todo::new().expect("Initialisation of db failed");
```

Now if we go back to the terminal and run a bunch of "add" command we should see our database correctly updating:
```console
$ cargo run -- add "make coffee"
todo saved

$ cargo run -- add "make pancakes"
todo saved

$ cat db.txt
make coffee     true
make pancakes   true
```



