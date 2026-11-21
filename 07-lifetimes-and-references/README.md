# Module 07: Lifetimes Demystified (For PHP Developers)

In PHP, you never think about how long a variable lives. The Zend Engine's Garbage Collector keeps variables alive in memory as long as at least one variable references them, and frees them when the reference count drops to 0.

In Rust, there is no garbage collector running in the background. To guarantee memory safety at compile time, Rust uses **Lifetimes**.

---

## 1. What is a Lifetime?

> **A lifetime is simply the scope for which a reference (`&T`) is valid.**

The Rust compiler includes a subsystem called the **Borrow Checker**. The borrow checker compares the lifetime of the data with the lifetime of the reference to make sure you **never have a dangling pointer** (a reference pointing to memory that has already been deallocated).

### The Dangling Pointer Problem (What Rust Prevents):

In C or C++:
```c
int* get_dangling_pointer() {
    int x = 42;
    return &x; // 💥 Disasters happen here! `x` is destroyed when function exits.
               // The pointer now points to random garbage or corrupted memory!
}
```

In PHP, this is prevented by the runtime copying the value or keeping it alive in the GC heap.  
In Rust, the compiler catches this **at compile time**:

```rust
fn get_dangling_reference() -> &i32 {
    let x = 42;
    // &x 
    // ❌ COMPILE ERROR: returns a reference to data owned by the current function!
}
```

---

## 2. The Lifetime Syntax (`'a`)

Lifetimes are written with an apostrophe followed by a short name, usually `'a`:
- `&i32` = A reference to an integer.
- `&'a i32` = A reference to an integer with an explicit lifetime named `'a`.
- `&'a mut i32` = A mutable reference with lifetime `'a`.

> **CRITICAL INSIGHT:**  
> Lifetime annotations do **NOT** change how long a variable lives!  
> You cannot extend or shorten a variable's life by writing `'a`. Lifetime annotations merely **describe the relationship between the lifetimes of multiple references** so the compiler can verify them.

---

## 3. When Do You Need Lifetime Annotations?

Most of the time, Rust infers lifetimes automatically (called **Lifetime Elision**). You only need explicit lifetime annotations when a function takes **two or more references** and returns a reference, and the compiler cannot deduce which input reference is being returned.

### The Classic Example: Finding the Longest String

```rust
// ❌ FAILS TO COMPILE WITHOUT LIFETIMES:
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
// Error: missing lifetime specifier! Rust doesn't know if the returned &str borrows from x or y!

// ✅ COMPILES WITH LIFETIME ANNOTATIONS:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### What `longest<'a>` tells the compiler:
1. `x` and `y` must both live at least as long as lifetime `'a`.
2. The returned `&str` reference will live at most as long as the **shorter** of `x` and `y`.

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {result}"); // ✅ OK: string2 is still in scope!
    }
    // println!("{result}"); 
    // ❌ COMPILE ERROR: `string2` does not live long enough!
}
```

---

## 4. Lifetimes in Structs

If a struct holds **owned data** (like `String` or `i32`), it does **NOT** need lifetime annotations:
```rust
struct Client {
    name: String, // Owned data! No lifetime needed!
    balance: f64,
}
```

If a struct holds a **borrowed reference**, you must tell Rust that the struct cannot outlive the data it references:

```rust
struct ClientView<'a> {
    account_code: &'a str, // Borrows from an existing string
    role: &'a str,
}

fn main() {
    let raw_code = String::from("CL-9988");
    let view = ClientView {
        account_code: &raw_code,
        role: "admin",
    };

    println!("Viewing: {} ({})", view.account_code, view.role);
}
```

> **💡 Best Practice for PHP Developers:**  
> When starting out in Rust, **prefer owned types (`String`, `Vec<T>`) inside your structs**. Only use borrowed references (`&str`) inside structs when you are doing performance-critical, zero-copy parsing.

---

## 5. The `'static` Lifetime

`'static` is a special reserved lifetime meaning the data is available for the **entire duration of the program execution**.

Examples:
1. Hardcoded string literals:
   ```rust
   let s: &'static str = "Hello, world!";
   ```
   String literals are baked directly into the compiled binary executable's read-only memory (`.rodata` section), so they never get dropped.
2. Global constants:
   ```rust
   const APP_VERSION: &'static str = "1.0.0";
   ```

---

## 🛠️ Exercises: Module 07

1. **Prefix Matcher**:
   Write a function `first_matching<'a>(prefix: &str, str1: &'a str, str2: &'a str) -> Option<&'a str>` that returns the first string that starts with `prefix` (using `.starts_with(prefix)`).
2. **Document Excerpt Struct**:
   Create a struct `Excerpt<'a>` that contains a field `part: &'a str`. In `main()`, instantiate a sentence `String`, take a slice of the first 5 characters (`&sentence[0..5]`), and store it in an `Excerpt`.
