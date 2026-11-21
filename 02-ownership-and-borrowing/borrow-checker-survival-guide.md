# The Borrow Checker Survival Guide (For PHP Developers)

When learning Rust, 90% of beginners get frustrated and want to quit during their first two weeks because of one thing: **The Borrow Checker**.

In PHP, you can pass variables anywhere, modify them, reference them, or return them without thinking. In Rust, the compiler is your strict pair-programmer ensuring zero memory leaks and zero data races.

Here are the **Top 5 Borrow Checker Errors** PHP developers encounter, why your PHP instincts cause them, and how to fix them effortlessly.

---

## Error 1: "Use of moved value"

### The Broken Code:
```rust
fn print_message(msg: String) {
    println!("{msg}");
}

fn main() {
    let greeting = String::from("Hello, Laravel!");
    print_message(greeting);
    println!("Again: {greeting}"); // ❌ ERROR: borrow of moved value: `greeting`
}
```

### The PHP Instinct:
In PHP, passing `$greeting` passes a copy or handle; `$greeting` is still available in the outer scope:
```php
<?php
function printMessage(string $msg) { echo $msg; }
$greeting = "Hello, Laravel!";
printMessage($greeting);
echo "Again: " . $greeting; // Perfectly valid in PHP
```

### The Fix: Borrow instead of moving!
Don't give away ownership (`String`); pass a reference (`&str` or `&String`):
```rust
fn print_message(msg: &str) { // Borrow with &str
    println!("{msg}");
}

fn main() {
    let greeting = String::from("Hello, Laravel!");
    print_message(&greeting); // Lend a reference with &
    println!("Again: {greeting}"); // ✅ Works! greeting was never moved.
}
```

---

## Error 2: "Cannot borrow as mutable more than once"

### The Broken Code:
```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    let ref1 = &mut numbers;
    let ref2 = &mut numbers; // ❌ ERROR: cannot borrow `numbers` as mutable more than once at a time

    ref1.push(4);
    ref2.push(5);
}
```

### The PHP Instinct:
In PHP, multiple variables can point to the same array or object reference:
```php
<?php
$a = [1, 2, 3];
$ref1 = &$a;
$ref2 = &$a; // Multiple mutable references are allowed in PHP
$ref1[] = 4;
$ref2[] = 5;
```

### Why Rust Blocks It:
If two threads or two code branches have simultaneous mutable access, you get **data races and memory corruption**.

### The Fix: Scope your mutations sequentially!
Rust only allows **one mutable reference at any given instant**. Finish using the first reference before creating the second:
```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    numbers.push(4); // Direct mutation
    numbers.push(5); // Sequential mutation

    println!("{:?}", numbers); // [1, 2, 3, 4, 5]
}
```

---

## Error 3: "Cannot borrow as mutable because it is also borrowed as immutable" (Reading While Writing)

### The Broken Code:
```rust
fn main() {
    let mut users = vec!["Alice", "Bob", "Charlie"];

    for user in &users { // Immutable borrow (&users) begins here
        if *user == "Alice" {
            users.push("Dave"); // ❌ ERROR: cannot borrow `users` as mutable while borrowed as immutable!
        }
    }
}
```

### The PHP Instinct:
In PHP, modifying an array while looping over it is common (though it can lead to subtle bugs or infinite loops):
```php
<?php
$users = ["Alice", "Bob", "Charlie"];
foreach ($users as $user) {
    if ($user === "Alice") {
        $users[] = "Dave"; // Allowed in PHP!
    }
}
```

### Why Rust Blocks It:
When `users.push("Dave")` runs, the vector may need to reallocate its underlying heap memory to grow. If that happens, the memory address moves! The loop's `user` reference would now point to **freed / invalid memory**!

### The Fix: Collect new items first, or iterate over an index/copy!
```rust
fn main() {
    let mut users = vec![String::from("Alice"), String::from("Bob")];
    let mut new_users = Vec::new();

    for user in &users {
        if user == "Alice" {
            new_users.push(String::from("Dave"));
        }
    }

    // Now safely append outside the read loop
    users.extend(new_users);
    println!("{:?}", users);
}
```

---

## Error 4: "Cannot move out of a shared reference"

### The Broken Code:
```rust
struct User {
    name: String,
}

fn process_user(user: &User) {
    // let user_name = user.name; 
    // ❌ ERROR: cannot move out of `user.name` which is behind a shared reference
}
```

### The PHP Instinct:
In PHP, `$userName = $user->name` copies the string.

### The Fix: Clone it, or borrow it!
Option A: If you just need to inspect or print it, **borrow it**:
```rust
fn process_user(user: &User) {
    let user_name: &str = &user.name; // Borrowed! Zero allocation.
    println!("User name: {user_name}");
}
```

Option B: If you genuinely need a separate owned copy to store elsewhere, **clone it**:
```rust
fn process_user(user: &User) {
    let user_name: String = user.name.clone(); // Explicit deep copy
}
```

---

## Error 5: "Borrowed value does not live long enough" (Returning a Local Reference)

### The Broken Code:
```rust
fn get_greeting() -> &String {
    let s = String::from("Welcome");
    // &s 
    // ❌ ERROR: returns a reference to data owned by the current function!
}
```

### The PHP Instinct:
In PHP, variables created inside functions and returned are preserved in the GC heap.

### The Fix: Return the owned value!
Transfer ownership to the caller:
```rust
fn get_greeting() -> String { // Return owned String, not &String
    let s = String::from("Welcome");
    s // Ownership moves cleanly to the caller!
}
```

---

## 🎯 The "Rules of Thumb" for Fast Learning

1. **In Function Parameters**: Always default to `&str` for strings and `&[T]` for slices. Only take `String` or `Vec<T>` if the function needs to store or consume the data.
2. **When in Doubt, `.clone()` during early learning**: Don't let the borrow checker stall your progress. When prototyping, call `.clone()`. Once your program compiles and runs, you can refactor `.clone()` into references for speed.
3. **Rust compiler errors are tutorials, not insults**: Read the entire compiler output. Rust is unique: **it often gives you the exact line of code to fix the problem!**
