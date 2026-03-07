# Scala to Rust Cheat Sheet

## Types & Data

| Concept | Scala | Rust |
|---------|-------|------|
| Immutable binding | `val x = 5` | `let x = 5;` |
| Mutable binding | `var x = 5` | `let mut x = 5;` |
| Type annotation | `val x: Int = 5` | `let x: i32 = 5;` |
| String | `String` | `String` (owned), `&str` (borrowed) |
| Tuple | `(1, "hi")` | `(1, "hi")` |
| Unit (void) | `Unit` / `()` | `()` |
| Nullable | `null` (avoid) | Does not exist |
| Nothing/Never | `Nothing` | `!` (never type) |

## Common Numeric Types

| Scala | Rust |
|-------|------|
| `Int` | `i32` |
| `Long` | `i64` |
| `Float` | `f32` |
| `Double` | `f64` |
| `Boolean` | `bool` |
| `Byte` | `u8` / `i8` |

## Data Structures

```scala
// Scala
case class User(name: String, age: Int)
val u = User("Ada", 30)
```

```rust
// Rust
struct User {
    name: String,
    age: i32,
}
let u = User { name: String::from("Ada"), age: 30 };
```

| Concept | Scala | Rust |
|---------|-------|------|
| Product type | `case class Foo(x: Int)` | `struct Foo { x: i32 }` |
| Sum type | `sealed trait` + `case class/object` | `enum Foo { A, B(i32) }` |
| Enum (simple) | `enum Color { Red, Blue }` (Scala 3) | `enum Color { Red, Blue }` |
| Enum (with data) | `case class Some(x: Int) extends Option` | `enum Option { Some(i32), None }` |
| Tuple struct | N/A | `struct Point(f64, f64);` |

## Pattern Matching

```scala
// Scala
x match {
  case Some(value) => println(value)
  case None        => println("nothing")
}
```

```rust
// Rust
match x {
    Some(value) => println!("{}", value),
    None        => println!("nothing"),
}
```

| Pattern | Scala | Rust |
|---------|-------|------|
| Wildcard | `case _ =>` | `_ =>` |
| Binding | `case x =>` | `x =>` |
| Guard | `case x if x > 0 =>` | `x if x > 0 =>` |
| Destructure | `case (a, b) =>` | `(a, b) =>` |
| Nested | `case Foo(Bar(x)) =>` | `Foo(Bar(x)) =>` |
| Exhaustive? | Warning (sealed) | **Error** (compiler enforced) |

## Option & Result

| Concept | Scala | Rust |
|---------|-------|------|
| Optional value | `Option[T]` | `Option<T>` |
| Present | `Some(x)` | `Some(x)` |
| Absent | `None` | `None` |
| Success/Failure | `Either[E, A]` or `Try[T]` | `Result<T, E>` |
| Right/Success | `Right(x)` | `Ok(x)` |
| Left/Failure | `Left(e)` | `Err(e)` |
| Get or crash | `.get` | `.unwrap()` |
| Get or default | `.getOrElse(d)` | `.unwrap_or(d)` |
| Map | `.map(f)` | `.map(f)` |
| FlatMap | `.flatMap(f)` | `.and_then(f)` |
| Propagate error | N/A (for-comp) | `?` operator |

### The ? operator (Rust's killer feature for errors)

```scala
// Scala: for-comprehension to chain Either
for {
  config <- loadConfig()
  conn   <- connect(config)
  data   <- fetchData(conn)
} yield data
```

```rust
// Rust: ? propagates Err automatically
fn do_stuff() -> Result<Data, AppError> {
    let config = load_config()?;
    let conn = connect(&config)?;
    let data = fetch_data(&conn)?;
    Ok(data)
}
```

## Traits / Interfaces

```scala
// Scala
trait Greet {
  def hello(&self) -> String
}
case class User(name: String) extends Greet {
  def hello: String = s"Hi, $name"
}
```

```rust
// Rust
trait Greet {
    fn hello(&self) -> String;
}
struct User { name: String }
impl Greet for User {
    fn hello(&self) -> String {
        format!("Hi, {}", self.name)
    }
}
```

| Concept | Scala | Rust |
|---------|-------|------|
| Define behavior | `trait Foo` | `trait Foo` |
| Implement | `extends Foo` / `with Foo` | `impl Foo for Bar` |
| Type class derivation | `derives Codec` (Scala 3) | `#[derive(Debug, Clone)]` |
| Generic bound | `[T: Foo]` (context bound) | `<T: Foo>` or `impl Foo` |
| Multiple bounds | `[T: Foo: Bar]` | `<T: Foo + Bar>` |
| Associated types | `type X` in trait | `type X;` in trait |

Key difference: In Scala, impl is inline (`class Foo extends Bar`). In Rust, impl is a separate block (`impl Bar for Foo`). This means you can implement traits for types you didn't write.

## Functions

```scala
// Scala
def add(a: Int, b: Int): Int = a + b
```

```rust
// Rust
fn add(a: i32, b: i32) -> i32 { a + b }
```

| Concept | Scala | Rust |
|---------|-------|------|
| Declaration | `def foo(): Unit` | `fn foo()` |
| Return type | `def foo(): Int` | `fn foo() -> i32` |
| Last expression returns | Yes | Yes (no semicolon!) |
| Explicit return | `return x` | `return x;` (rare) |
| Lambda | `(x: Int) => x + 1` | `\|x: i32\| x + 1` |
| Method on type | `def foo(self)` in class | `fn foo(&self)` in `impl` block |

**Semicolons:** Rust uses semicolons. But the last expression in a block without a semicolon is the return value. Adding a semicolon makes it return `()` instead.

```rust
fn five() -> i32 {
    5       // returns 5 (no semicolon)
}
fn oops() -> () {
    5;      // semicolon makes this return ()
}
```

## Collections

| Scala | Rust | Notes |
|-------|------|-------|
| `List[T]` | `Vec<T>` | Growable array |
| `Map[K, V]` | `HashMap<K, V>` | Needs `use std::collections::HashMap` |
| `Set[T]` | `HashSet<T>` | Needs `use std::collections::HashSet` |
| `Array[T]` | `[T; N]` | Fixed-size array |
| `Seq[T]` (slice) | `&[T]` | Borrowed view of a Vec or array |
| `.map(f)` | `.iter().map(f)` | Rust needs `.iter()` first |
| `.filter(f)` | `.iter().filter(f)` | Same |
| `.flatMap(f)` | `.iter().flat_map(f)` | Same |
| `.foreach(f)` | `.iter().for_each(f)` or `for x in &v` | for loops are idiomatic |
| `.toList` | `.collect::<Vec<_>>()` | Turbofish syntax `::<>` |

## Printing & Formatting

| Scala | Rust |
|-------|------|
| `println(x)` | `println!("{}", x)` |
| `println(s"hello $name")` | `println!("hello {}", name)` or `println!("hello {name}")` |
| `x.toString` | `format!("{}", x)` (needs `Display` trait) |
| Debug print | N/A | `println!("{:?}", x)` (needs `Debug` trait) |
| Pretty debug | N/A | `println!("{:#?}", x)` |

`!` means it's a **macro**, not a function. Macros can take variable arguments (Rust functions can't).

## Ownership (Rust-only, no Scala equivalent)

This is the big new concept. Scala uses garbage collection. Rust uses ownership rules enforced at compile time.

| Rule | What it means |
|------|---------------|
| Each value has one owner | Like having exactly one `val` pointing to it |
| When the owner goes out of scope, value is dropped | Like GC, but deterministic |
| You can **borrow** with `&` (shared) | Read-only reference, many allowed |
| You can **borrow** with `&mut` (exclusive) | Mutable reference, only one at a time |
| You can **move** ownership | Original variable can no longer be used |

```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 is MOVED to s2, s1 is now invalid
// println!("{}", s1); // COMPILE ERROR: s1 was moved

let s3 = s2.clone();  // explicit copy, both valid
```

**Scala equivalent?** There isn't one. In Scala, `val s2 = s1` just copies the reference and both remain valid (GC tracks them). In Rust, the compiler tracks ownership instead of a runtime GC.

## Modules & Imports

| Concept | Scala | Rust |
|---------|-------|------|
| Import | `import foo.bar.Baz` | `use foo::bar::Baz;` |
| Wildcard import | `import foo._` / `import foo.*` | `use foo::*;` |
| Rename import | `import foo.{Bar => B}` | `use foo::Bar as B;` |
| Module file | `package foo` | `mod foo;` (in parent) |
| Visibility | `private` / `protected` | `pub` (default is private) |

Rust default is private (opposite of Scala). You explicitly `pub` what you want to expose.

## Testing

| Concept | Scala | Rust |
|---------|-------|------|
| Test framework | ScalaTest / MUnit | Built into cargo |
| Test location | `src/test/scala/...` | Same file, `#[cfg(test)] mod tests` |
| Test annotation | `test("name")` | `#[test]` |
| Assert equals | `assertEquals(a, b)` | `assert_eq!(a, b)` |
| Assert | `assert(condition)` | `assert!(condition)` |
| Run tests | `sbt test` | `cargo test` |
| Run one test | `sbt "testOnly *MyTest"` | `cargo test test_name` |

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

`#[cfg(test)]` means "only compile this module when running tests." `use super::*` imports everything from the parent module (the file this test lives in).

## Error Handling Comparison

| Approach | Scala | Rust |
|----------|-------|------|
| Exceptions | `throw new Exception("boom")` | Does not exist (no exceptions!) |
| Catching | `try/catch` | N/A |
| Typed errors | `Either[Error, Value]` | `Result<Value, Error>` |
| Custom errors | `case class MyError extends Exception` | `enum MyError` + `impl Display/Error` |
| Panic (crash) | `throw` | `panic!("boom")` (avoid in libraries) |
| Unwrap (crash if Err) | `.get` on Option | `.unwrap()` |

Rust has **no exceptions**. Every error must be in the return type. The compiler forces you to handle it. `?` makes this ergonomic. `panic!` exists but is for unrecoverable bugs, not business logic.

## Build & Tooling

| Task | Scala (sbt) | Rust (cargo) |
|------|-------------|--------------|
| Build | `sbt compile` | `cargo build` |
| Run | `sbt run` | `cargo run` |
| Test | `sbt test` | `cargo test` |
| Lint | scalafix/wartremover | `cargo clippy` |
| Format | scalafmt | `cargo fmt` |
| Docs | `sbt doc` | `cargo doc` |
| Publish | `sbt publish` (to Maven) | `cargo publish` (to crates.io) |
| REPL | `sbt console` | None built-in (use `evcxr`) |
| Deps file | `build.sbt` | `Cargo.toml` |
| Lock file | N/A (sbt has resolution) | `Cargo.lock` |
| Run with args | `sbt "run arg1 arg2"` | `cargo run -- arg1 arg2` |

## Derive Macros (Scala 3 derives equivalent)

```scala
// Scala 3
case class User(name: String, age: Int) derives Codec, Show
```

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}
```

Common derives:

| Rust derive | What it gives you | Scala equivalent |
|-------------|-------------------|------------------|
| `Debug` | `{:?}` formatting | `.toString` on case class |
| `Clone` | `.clone()` method | `.copy()` on case class |
| `PartialEq` | `==` comparison | Built into case class |
| `Serialize` | JSON serialization (serde) | `derives Codec` / circe |
| `Deserialize` | JSON deserialization (serde) | `derives Codec` / circe |
| `Parser` | CLI arg parsing (clap) | N/A |