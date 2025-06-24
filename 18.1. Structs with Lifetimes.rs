/*
In Rust, when we define a struct that holds references,
we need to explicitly tell the compiler about the *lifetimes*
of those references.

Why? Because references must always be valid for the duration
they are used. If a struct contains a reference, Rust wants
to ensure that the struct doesn't outlive the data it points to.
*/

// This won't compile because the compiler doesn't know how long
// the reference `&str` in the struct `User` will live.
/*
struct User {
    name: &str  // Error: missing lifetime specifier
}
*/

// Correct version using *lifetime annotations*:
struct User<'a> {
    // This tells the compiler: the reference `name` inside `User`
    // must not outlive the lifetime `'a`.
    name: &'a str,
}

fn main() {
    // We create a `String` on the heap.
    let first_name = String::from("Tushar");

    // We create a reference to that string and store it inside the struct.
    // This is safe because both `user` and `first_name` live in the same scope.
    let user = User { name: &first_name };

    // This is valid because `first_name` is still in scope here.
    println!("The name of the user is: {}", user.name);
}

/*
Explanation:

- The struct `User` contains a reference (`&str`).
- In Rust, references must be valid. The compiler ensures this using lifetimes.
- `'a` is a lifetime parameter. It represents "some lifetime" that will be decided when the struct is used.
- The compiler now knows that the `name` field in `User` must live as long as `'a`.

Think of it like this:
"If `user` lives for `'a`, then the reference `name` must also be valid for `'a`."

This ensures memory safety: we never use a reference after the data it's pointing to has been dropped.
*/
