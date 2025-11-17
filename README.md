# Eon Programming Language

Eon is a small, statically-typed, expression-oriented language that compiles to bytecode and runs on a custom VM. It features algebraic data types, pattern matching, traits (interfaces), and a standard prelude. Native capabilities are provided via Rust FFI modules (os, time, random, term).

## Highlights
- Statically-typed with type inference
- Algebraic data types and pattern matching
- Interfaces (traits) with implementations
- Expression- and block-bodied functions
- Modules for OS/time/random/terminal via Rust FFI
- Runs on a compact bytecode VM with automatic GC

## How it works (high level)
1. Parse: `.en` source is parsed into an AST (`eon_core::parse`)
2. Statics: name resolution + type checking + inference (`eon_core::statics`)
3. Translate: typed AST → VM bytecode (`eon_core::translate_bytecode`)
4. Run: a stack-based VM executes the program (`eon_core::vm`)

The CLI (`eon`) ties this together: it reads your `.en`, loads the prelude, resolves imports, compiles to bytecode, and runs the VM loop, handling pending host functions.

## Memory management
- Automatic (GC-managed): you never `free` or `drop` objects in Eon.
- The VM owns heap objects (strings/arrays/structs/variants). GC is invoked periodically during the run loop.
- FFI layer marshals values across Rust boundaries safely; no manual lifetime work is needed from Eon programs.
- Resource lifetimes (files, processes) should be managed explicitly via APIs; destruction is not deterministic (typical of GC).

## Syntax at a glance
```eon
fn not(b: bool) = if b false else true

// ADTs + pattern matching
type option<T> = some(T) | none
fn unwrap(m: option<T>) -> T {
  match m {
    .some(x) -> x,
    .none -> panic("cannot unwrap option.none")
  }
}

// Interfaces (traits) and impls
interface ToString { fn str: Self -> string }
implement ToString for int { fn str(n) = int_to_string(n) }

// Loops and blocks
fn fib(n) {
  var x = 0
  var y = 1
  while n > 0 {
    let t = x
    x = y
    y = t + y
    n = n - 1
  }
  x
}
for i in range(10) { println(i & ": " & fib(i)) }
```

## FFI and modules
Eon exposes native features through Rust crates compiled as modules. The FFI bindings are generated automatically.
- `modules/os` – files: fread/fwrite/fexists/...
- `modules/time` – get_time()/sleep()
- `modules/random` – random_int()/random_float()
- `modules/term` – basic terminal control/input

## Project layout
- `eon_core/` – parser, type checker, bytecode translator, VM
- `eon_cli/` – command-line tool (`eon`) that compiles and runs Eon programs
- `modules/` – Rust-backed standard modules (os, time, random, term)
- `examples/` – sample Eon programs

## Build and run

```powershell
# Build release
cargo build --release

# Run a program without modules
.\target\release\eon.exe examples\fib.en

# Run examples that use FFI modules (random/time/os/term)
# Provide the modules dir and the shared objects dir
.\target\release\eon.exe --modules modules --shared-objects target\release examples\random_walk.en
.\target\release\eon.exe --modules modules --shared-objects target\release examples\snake.en
# (On Unix-like shells, replace backslashes with slashes)
```


## Running the examples

- Non-FFI examples (no flags required):
  - `./target/release/eon.exe examples/fib.en`
  - `./target/release/eon.exe examples/forloop.en`
  - `./target/release/eon.exe examples/range.en`
  - `./target/release/eon.exe examples/pattern_match_tree.en`

- FFI-dependent examples (require modules and shared objects flags):
  - `./target/release/eon.exe --modules modules --shared-objects target/release examples/random_walk.en`
  - `./target/release/eon.exe --modules modules --shared-objects target/release examples/file_copy_demo.en`
  - `./target/release/eon.exe --modules modules --shared-objects target/release examples/snake.en` (interactive)

Notes on module scoping:
- Using `use os` imports the module’s functions directly into the current scope. Call them unqualified, e.g., `fexists`, `fread`, `fwrite` (not `os.fexists`).
- The `examples/file_copy_demo.en` demonstrates this pattern and has been updated accordingly.
- On Unix-like shells, replace backslashes with slashes in paths.

## Testing
```powershell
# All tests in the workspace
cargo test --all

# Core language tests only
cargo test -p eon_core

# Focused sets
cargo test -p eon_core typecheck      # type system tests
cargo test -p eon_core e2e_bytecode   # end-to-end VM tests
```

## Safety and guidelines
- Unsafe code is confined to FFI boundaries and VM internals
- All unsafe points document required preconditions (e.g., valid VM pointer)
- No glob re-exports; no problematic static globals

## When to use Eon
- Embeddable scripting with strong static types
- Lightweight programs with pattern matching and ADTs
- Extending with native features through simple Rust modules

---
For a quick start, explore the `examples/` folder and the FFI-backed `modules/` APIs.