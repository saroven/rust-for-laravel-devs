# PHP to Rust: The Complete Developer Dictionary

This reference bridges PHP 8+ and Laravel constructs to their modern Rust equivalents.

---

## 1. Core Syntax & Variables

| Concept | PHP 8+ | Rust 2021+ | Notes |
|---|---|---|---|
| **Immutable variable** | ❌ (PHP has no immutable variables) | `let x = 5;` | In Rust, variables are immutable by default. |
| **Mutable variable** | `$x = 5; $x = 10;` | `let mut x = 5; x = 10;` | Must use `mut` keyword. |
| **Constant** | `const TAX_RATE = 0.15;` | `const TAX_RATE: f64 = 0.15;` | Explicit type annotation required. |
| **Type Casting** | `(int) $val` | `val as i32` or `val.parse::<i32>()` | `as` is primitive cast; `.parse()` handles string conversion safely. |
| **Shadowing** | ❌ (Overwrites same variable) | `let val = "42"; let val: u32 = val.parse()?;` | Re-declaring `let` changes type without dummy variable names. |
| **String Interpolation** | `"Hello, {$name}!"` | `format!("Hello, {name}!")` | `println!("Hello, {name}!");` for stdout. |
| **Dump and Die** | `dd($user);` or `dump($user);` | `dbg!(&user);` or `println!("{:#?}", user);` | `dbg!` prints file and line number, and returns the expression! |

---

## 2. Nullability & Values

| Concept | PHP | Rust |
|---|---|---|
| **Nullable type** | `?string` | `Option<String>` |
| **Null literal** | `null` | `None` |
| **Non-null value** | `'John'` | `Some(String::from("John"))` |
| **Null coalescing** | `$name ?? 'Default'` | `name.unwrap_or_else(\|\| "Default".to_string())` |
| **Nullsafe operator** | `$user?->profile?->avatar` | `user.and_then(\|u\| u.profile).map(\|p\| p.avatar)` |
| **Checking presence** | `is_null($x)` or `$x !== null` | `x.is_none()` or `x.is_some()` |

---

## 3. Data Structures & OOP

| PHP / Laravel | Rust Equivalent | Architectural Insight |
|---|---|---|
| `class User { public string $name; }` | `struct User { pub name: String }` | Rust structs hold only data, never method bodies. |
| Class Methods | `impl User { pub fn new(...) -> Self { ... } }` | Behavior is implemented separately via `impl` blocks. |
| `$this->name` | `self.name` | Methods explicitly take `&self` (read) or `&mut self` (write). |
| Constructor `__construct` | Associated function convention: `User::new(...)` | Rust has no constructor syntax; `new` is an idiomatic function returning `Self`. |
| `interface HasUuid` | `trait HasUuid { fn uuid(&self) -> String; }` | Traits define interface contracts. |
| `trait Timestampable` | `trait Timestampable` (with default methods) | Rust traits can provide default method implementations. |
| Class Inheritance (`extends`) | ❌ **Does not exist in Rust** | Use **Composition**: embed one struct inside another, or use Trait bounds. |
| PHP 8.1 Enum | `enum Status: string { case Active; }` | `enum Status { Active, Suspended }` |
| Enum with data (Algebraic Data Types)| ❌ Impossible without complex classes | `enum Event { Login(UserId), Error { code: u16, msg: String } }` |

---

## 4. Error Handling

| Concept | PHP | Rust |
|---|---|---|
| **Strategy** | Exceptions (`try / catch / throw`) | Explicit return values: `Result<T, E>` (`Ok(T)` or `Err(E)`) |
| **Throw exception** | `throw new InvalidArgumentException("Bad");` | Return `Err(AppError::Invalid("Bad".into()))` |
| **Try / Catch** | `try { ... } catch (Exception $e) { ... }` | `match result { Ok(val) => ..., Err(e) => ... }` |
| **Propagate error up** | Automatic bubbling (can crash unhandled) | `let val = do_something()?;` (The `?` operator) |
| **Fatal Crash / Abort**| `die("Fatal");` or `exit(1);` | `panic!("Fatal crash message");` |

---

## 5. Arrays, Collections & Functional Pipelines

| PHP / Laravel | Rust Equivalent | Architectural Insight |
|---|---|---|
| `[1, 2, 3]` (indexed) | `vec![1, 2, 3]` (`Vec<i32>`) | Dynamic array stored on heap. All elements must have the same type. |
| `['a' => 1, 'b' => 2]` | `HashMap::from([("a", 1), ("b", 2)])` | Standard key-value hash map. |
| `collect($arr)->map(...)` | `arr.iter().map(...).collect()` | Rust iterators are lazy: zero intermediate memory allocations. |
| `collect($arr)->filter(...)` | `arr.into_iter().filter(...)` | Chains without evaluation until a consumer like `.collect()` is called. |
| `in_array($val, $arr)` | `arr.contains(&val)` | Efficient binary or linear search. |
| `count($arr)` | `arr.len()` | Returns `usize` (64-bit integer). |
| `array_unique($arr)` | `let set: HashSet<_> = arr.into_iter().collect();` | Fast set deduplication. |

---

## 6. Laravel Web Stack vs Rust Axum Stack

| Laravel Layer | Rust (Axum + Tokio) Equivalent |
|---|---|
| **Routing** (`routes/api.php`) | `Router::new().route("/users", get(get_users).post(create_user))` |
| **Controller** | Async handler: `async fn get_users(State(db): State<Pool>) -> Json<Vec<User>>` |
| **FormRequest / Validation** | `Json(payload): Json<CreateUserDto>` using `validator` crate |
| **Middleware** | Tower layers / Axum middleware (`axum::middleware::from_fn(auth_guard)`) |
| **Service Container / DI** | `State(state): State<Arc<AppState>>` |
| **Eloquent Model** | `sqlx::FromRow` struct or `sea-orm` ActiveModel |
| **Database Migrations** | `sqlx migrate run` or `sea-orm-cli migrate up` |
| **Artisan Command** | `clap` CLI crate (`#[derive(Parser)]`) |
| **Queue Worker / Jobs** | Tokio asynchronous channels (`tokio::sync::mpsc`) or Redis workers |
| **Log Facade (`Log::info`)** | `tracing` crate: `tracing::info!("User created: {id}");` |
| **Environment (`.env`)** | `dotenvy` crate + strongly typed Config struct with `envy` |
| **PHPUnit / Pest** | Built-in test runner: `cargo test` (`#[test]`) |
