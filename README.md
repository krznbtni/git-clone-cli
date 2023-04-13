# Git Clone CLI

## How should the first version behave?

1. The User should be met with an input prompt, querying for a GitHub username.
2. The Program should call the GitHub API, querying for repositories available repositories.
3. The User should be met with a multiselect prompt and select which repositories to clone.
4. The User should be met with an input prompt, querying for a directory to clone the repositories to (default: current directory).

## How should future versions behave?

- Allow the User to select from where to clone repositories (GitHub, Gitlab, etc).
- Figure out how to fetch non-public repositories from for example GitHub (private, organizational).
  - Will probably require a personal access token. If so, allow the User to configure one.

## Rust Notes

### Ownership

- Ownership, borrowing and slices ensure memory safety at compile time.

#### References and Borrowing

- The '&'-sign means that it is a REFERENCE.
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- Pass variables to functions without having the function take ownership by making the function's parameter a reference: `fn foo(x: &str) {}`.
-  The action of creating a reference is called 'borrowing'.
- References are immutable by default, meaning that you are not allowed to modify it.
- For a reference to be mutable, its definition must include the `mut` word: `let mut s = String::from("hello")`, and used like `&mut s`.
- Mutable references' big restriction is that you cannot create 2 mutable references to it.
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
... look up data races...

#### The Slice Type

- A string slice is a reference to part of a String.
- The type that signifies “string slice” is written as `&str`.
- **String literals** are slices pointing to that specific point of the binary.

**String Slices as Parameters:**
- if we have a string slice (`&str`), we'll be able to pass that into the function directly.
- if we have a String (`String`), we can pass a slice of that String or a reference to the String (`&var`).

### Functional Language Features: Iterators and Closures

#### Processing a Series of Items with Iterators

##### Iterator Adaptors

- A `map` is an iterator adaptor.
- It does not consume the iterator.
- It produces different iterators by changing some aspect of the original iterator.
- It returns a new iterator that produces the modified items.
- It's closure creates a new iterator.
- An iterator adaptor is _lazy_ and **needs** to be consumed.
- To consume the iterator, call the `collect` method.
- The `collect` method consumes the iterator and collects the values into a collection data type.


### Modules

Rust's module tree starts with the crate root. For a binary crate, that's **src/main.rs**. For a library crate, that's **src/lib.rs**.
Rust's compiler will begin looking through the crate root for modules.

If you want a module named **a** in your binary crate:

- create a file **src/a.rs**.
- type `mod a` at the top of **src/main.rs**.

#### Submodules across files

A module can have submodules to further organize code.
If you want module **a** to include module **b** and **c**:

- create the files **src/a/b.rs**, **src/a/c.rs**, **src/a/mod.rs**.
- type `mod b; mod c` in **src/a/mod.rs**.
- type `mod a;` in **src/main.rs**.

#### Visibility

- All items in modules are by default private.
- To make them **accessible** from the outside, prepend the function name with `pub`, like so: `pub fn x() {}`.
- To **use** the function in for example **src/main.rs**, use the **path qualifier (`::`)**: `a::x();`.
- Similar approach applies in submodules. If you want to use a function from module **b** in module **c**, type `super::b::foo();`.

#### Re-exporting Items

- Something defined in a submodule is **not** accessible to a non-parent module. For example, the function `foo` defined in module **b** is not accessible by **src/main.rs**. To make it accessible to **src/main.rs**, you need to **re-export** it by prepending `pub` to `mod b` in **src/a/mod.rs**.

#### The "use" declaration

To shorten the path when accessing a module, in **src/a/mod.rs**:
```rs
mod b;
mod c;

use b::do_b;
use c::do_c;
```

#### Cheat Sheet

- A module tree starts from the crate root.
- Use `mod` to build your tree with modules and submodules.
- Use `pub` to make module items visible to the parent module.
- You can re-export with `pub mod` or `pub use`.
