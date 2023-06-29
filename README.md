# RUSTY DO

Rusty Do is a simple todo list manager written in Rust.
I wrote it primarily to learn Rust.

It uses sqlite3 to store the todo list.
The todo list itself is stored in a file called `todo.db` in the current directory.

It uses a very simple schema

```sql
CREATE TABLE todo (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0
);
```

I found a sqlite wraper for Rust. Using the wraper, I was able to write a simple command line todo list manager.

It has the following commands
- add: Add a new todo item
- show: List all todo items
- mark: Mark a todo item as completed
- del: Delete a todo item
- exit: Exit the program

## Implementation

The program is implemented mostly using structs and match statements.
The sqlite3 wraper is used to execute sql statements.
This sqlite3 wraper is hence again wrapped in a struct that exists is the 
`src/utils.rs` file.
This struct is called `Db` and it has methods to add, show, mark and delete todo items.

The other `src/todo.rs` has a few handy methods to handle the TODO item.
The `src/main.rs` file has the main function that parses the input and calls the appropriate methods.

The whole program is very simple and easy to understand.
So, if you are a beginner in Rust, you can take a look at the code and understand it.
Some of the code is inspired from the [Rust Book](https://doc.rust-lang.org/book/).

## To Fix

- [ ] Add a way to edit a todo item
- [ ] Fix the way the todo list is displayed
- [ ] Add a way to search for a todo item
- [ ] Migrate from using `sqlite::excecute` to `sqlite::prepare`

## Installation

For installing prebuilt binaries, you can download the latest release from the [releases](
/releases) page.

## Building

```bash
git clone https://github.com/newtoallofthis123/rustydo.git
cd rustydo
cargo install --path .
cargo build --release
```

After this, you can run the program using

```bash
cargo run --release
```

You will be able to find the actual executable in the `target/release/rust-todo.exe`.

## Usage

```bash
rustydo
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

MIT License

Copyright (c) 2023 Ishan Joshi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

> NoobScience 2023