# **Variable Declaration and Type Inference in Rust**

In Rust, variable declaration involves specifying a name and optionally a data type. Here's a breakdown of different declaration styles:

- **`let` (Immutable Variables):** This is the default way to declare variables in Rust. The compiler infers the data type if not explicitly provided. Once declared, `let` variables cannot be reassigned.

```rust
let x = 10;    // Inferred type: i32
let message = "Hello, world!"; // Inferred type: &str (string slice)
```

- **`let mut` (Mutable Variables):** Use `let mut` for variables that need to be changed later.

```rust
let mut count = 0;
count += 1; // Increment the value of count
```

- **Explicit Type Annotation:** While type inference is generally encouraged, you can explicitly specify the type for clarity or to constrain the variable to a specific data type.

```rust
let value: i64 = 9223372036854775807; // Explicitly assign i64 type
```

**Numeric Data Types in Rust**

Rust provides a variety of integer and floating-point data types to represent different number ranges and precisions. Here's a table summarizing the commonly used types:

| Type        | Size (bits) | Description                                            |
|--------------|-------------|---------------------------------------------------------|
| `i8`          | 8           | Signed integer, range: -128 to 127                        |
| `i16`         | 16          | Signed integer, range: -32,768 to 32,767                 |
| `i32`         | 32          | Signed integer, range: -2,147,483,648 to 2,147,483,647 (default for `i` if not specified) |
| `i64`         | 64          | Signed integer, range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `u8`          | 8           | Unsigned integer, range: 0 to 255                         |
| `u16`         | 16          | Unsigned integer, range: 0 to 65,535                         |
| `u32`         | 32          | Unsigned integer, range: 0 to 4,294,967,295 (default for `u` if not specified) |
| `u64`         | 64          | Unsigned integer, range: 0 to 18,446,744,073,709,551,615 |
| `f32`         | 32          | Single-precision floating-point number (approx. 7 decimal digits of precision) |
| `f64`         | 64          | Double-precision floating-point number (approx. 15 decimal digits of precision) |

**Boolean Data Type (bool)**

In Rust, the `bool` data type represents Boolean values: `true` or `false`. It's commonly used for conditional statements and decision-making.

```rust
let is_raining = true;
let is_sunny = !is_raining; // Logical negation (not)

if is_raining {
  println!("It's raining, bring an umbrella!");
} else {
  println!("Enjoy the sunshine!");
}
```

**String Data Type**

Represents a dynamically allocated, mutable string of characters. Use functions like `from` to create strings.

```rust
let greeting = String::from("Hello, world!"); // Allocates memory for the string
greeting.push_str(" Have a nice day!"); // Mutable string modification
println!("{}", greeting); // Prints "Hello, world! Have a nice day!"
```
