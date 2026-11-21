# Module 02: Ownership, Borrowing & Lifetimes

This is the single most important module in Rust.  
In PHP, you never think about memory deallocation because the Zend Engine's Garbage Collector does it for you. In C, you must manually call `malloc` and `free` (risking memory leaks and crashes).

**Rust uses a third way: Compile-Time Ownership.**  
It delivers the memory safety of PHP with the blazing speed and zero runtime overhead of C.

---

## 1. Stack vs Heap (The Foundation)

Before understanding ownership, you must understand how data is stored in RAM:

| Property | The Stack | The Heap |
|---|---|---|
| **Speed** | Extremely fast (L1/L2 CPU cache friendly) | Slower (requires pointer lookup) |
| **Size** | Known, fixed size at compile time | Dynamic, unknown, or expandable at runtime |
| **Examples** | `i32`, `bool`, `f64`, fixed arrays `[i32; 4]` | `String`, `Vec<T>`, dynamic objects |
| **PHP Analogy**| Primitive scalars | PHP objects and associative arrays |

---

## 2. The Three Rules of Ownership

1. **Each value in Rust has an owner** (a variable).
2. **There can only be ONE owner at a time.**
3. **When the owner goes out of scope, the value is automatically dropped (freed from memory).**

There is no `unset($var)` or GC pause. When a variable hits `}`, Rust calls `drop()` automatically (RAII - Resource Acquisition Is Initialization).

---

## 3. Move Semantics: Where PHP Developers Get Confused

### How PHP works:
```php
<?php
$a = "Hello";
$b = $a; // Copy-on-write. Both $a and $b can still be read!
echo $a; // "Hello"
echo $b; // "Hello"
```

### How Rust works:
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // 💥 Ownership of the string MOVED to s2!

    // println!("{s1}");
    // ❌ COMPILE ERROR: borrow of moved value: `s1`
    
    println!("{s2}"); // ✅ Works! s2 is now the sole owner.
}
```

```
Memory representation during Move:

s1 (on Stack)                     Heap
[ ptr | len: 5 | cap: 5 ] ────► [ 'H', 'e', 'l', 'l', 'o' ]
                                     ▲
s2 (on Stack)                        │
[ ptr | len: 5 | cap: 5 ] ───────────┘
(s1 is invalidated! Only s2 can free the heap buffer)
```

Why? If both `s1` and `s2` owned the heap data, when they both went out of scope, Rust would try to free the same heap memory twice (**double free vulnerability**). By invalidating `s1`, Rust guarantees only `s2` drops the memory.

---

## 4. Copy Types (Stack-Only Data)

Simple types with a known fixed size on the stack implement the `Copy` trait. They are copied automatically rather than moved:

```rust
fn main() {
    let x = 42;
    let y = x; // Integers are Copy!
    println!("x: {x}, y: {y}"); // ✅ Both are valid!
}
```
Types that implement `Copy`: `i32`, `u64`, `f64`, `bool`, `char`, and tuples containing only `Copy` types.

---

## 5. References & Borrowing (Passing by Reference)

In PHP, when you pass an object to a function, PHP passes an object handle.  
In Rust, if you pass a `String` to a function, you **give away ownership**:

```rust
fn print_name(name: String) {
    println!("Name: {name}");
} // `name` is dropped here!

fn main() {
    let my_name = String::from("Tanvir");
    print_name(my_name);
    // println!("{my_name}"); // ❌ ERROR: my_name was moved into print_name!
}
```

### The Solution: Borrowing (`&` and `&mut`)

Instead of giving ownership, **lend** a reference:

### 1. Immutable Borrow (`&T`)
You can read the data, but cannot modify it:

```rust
fn print_name(name: &String) { // Borrows a reference
    println!("Name: {name}");
} // Reference goes out of scope, but the underlying data is NOT dropped!

fn main() {
    let my_name = String::from("Tanvir");
    print_name(&my_name); // Lend with &
    println!("Still have access: {my_name}"); // ✅ Perfectly valid!
}
```

### 2. Mutable Borrow (`&mut T`)
You can modify the borrowed data:

```rust
fn add_prefix(name: &mut String) {
    name.insert_str(0, "Mr. ");
}

fn main() {
    let mut my_name = String::from("Tanvir");
    add_prefix(&mut my_name);
    println!("{my_name}"); // "Mr. Tanvir"
}
```

---

## 6. The Golden Rule of References

At any given point in time, you can have:

> **EITHER** any number of immutable references (`&T`)  
> **OR** exactly ONE mutable reference (`&mut T`)  
> **NEVER BOTH AT THE SAME TIME.**

### Example of what the compiler blocks:
```rust
fn main() {
    let mut data = String::from("Hello");

    let r1 = &data;     // Immutable borrow #1 (reader)
    let r2 = &data;     // Immutable borrow #2 (reader)
    // let r3 = &mut data; 
    // ❌ COMPILE ERROR: cannot borrow `data` as mutable because it is also borrowed as immutable!

    println!("{r1} and {r2}");
}
```

### Why this rule exists:
In PHP, if two processes or threads modify an array while another loops over it, you get corrupt states or crashes.  
Rust eliminates **Data Races** at compile time! A reader will never see half-written or mutating data.

---

## 7. `String` vs `&str` (The String Mystery Explained)

Coming from PHP, having two string types is confusing. Here is the distinction:

| Type | Name | Analogy | Characteristics |
|---|---|---|---|
| `String` | Owned String | `StringBuilder` / Allocated string | Stored on the heap, dynamically growable, owned. |
| `&str` | String Slice | Read-only view into string | A pointer + length borrowing a part of a `String` or hardcoded string literal (`"hello"`). |

```rust
fn greet(name: &str) { // 💡 BEST PRACTICE: accept &str in function parameters!
    println!("Hello, {name}!");
}

fn main() {
    let literal: &str = "World";
    let owned: String = String::from("Tanvir");

    greet(literal);      // Passes &str directly
    greet(&owned);       // Automatically coerces &String to &str!
}
```

---

## 🛠️ Exercises: Module 02

1. **Fix the Move**:
   ```rust
   fn main() {
       let account = String::from("ACC-10023");
       process_account(account);
       println!("Logged account: {account}"); // Fix this so it compiles!
   }
   fn process_account(acc: String) {
       println!("Processing {acc}");
   }
   ```
2. **Fix the Borrowing Conflict**:
   ```rust
   fn main() {
       let mut balance = 500;
       let ref1 = &balance;
       let ref2 = &mut balance;
       println!("Balance: {ref1}, Updated: {ref2}");
   }
   ```
   *Identify why the compiler rejects this and rewrite it safely.*
