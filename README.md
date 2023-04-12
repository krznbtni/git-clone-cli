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
