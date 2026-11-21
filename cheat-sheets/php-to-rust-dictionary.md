# PHP & Laravel to Rust: The Modern Rosetta Stone

Targeted for **Laravel 11+ / 12+ (PHP 8.3/8.4)** and **Modern Rust (Rust 1.80+, 2021/2024 Edition)**.

---

## 1. Modern Syntax & Declarations

| Concept | Modern Laravel / PHP (8.3/8.4) | Modern Rust (1.80+) | Modern Engineering Insight |
|---|---|---|---|
| **Immutable variable** | ❌ (PHP variables are always mutable) | `let x = 5;` | Rust defaults to immutability for thread-safety. |
| **Mutable variable** | `$x = 5; $x = 10;` | `let mut x = 5; x = 10;` | Explicit `mut` keyword required. |
| **Constants** | `const TIMEOUT = 30;` | `const TIMEOUT: u32 = 30;` | Evaluated at compile time. |
| **Type Casting** | `(int) $val` | `val as i32` or `val.parse::<i32>()?` | Primitive cast vs safe fallible string parsing. |
| **Variable Shadowing** | ❌ (Overwrites same slot) | `let val = "42"; let val: u32 = val.parse()?;` | Transform value and type without dummy names. |
| **String Interpolation** | `"Hello, {$name}!"` | `format!("Hello, {name}!")` | Inline variable capture in modern Rust. |
| **Dump and Die** | `dd($user);` | `dbg!(&user);` or `println!("{user:#?}");` | Modern inline formatting. |
| **Global Singletons** | Laravel Singleton in Service Provider | `static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(...);` | Built-in standard library in Rust 1.80+ (No third-party `lazy_static` needed!). |

---

## 2. Nullability, Guard Clauses & Early Returns

| Concept | Modern Laravel / PHP 8.3+ | Modern Rust (1.80+) |
|---|---|---|
| **Nullable type** | `?string` | `Option<String>` |
| **Null value** | `null` | `None` |
| **Has value** | `'John'` | `Some(String::from("John"))` |
| **Null coalescing** | `$name ?? 'Default'` | `name.unwrap_or_else(\|\| "Default".to_string())` |
| **Nullsafe operator** | `$user?->profile?->avatar` | `user.and_then(\|u\| u.profile).map(\|p\| p.avatar)` |
| **Early Guard Clause** | `if (!$user) return response()->json([], 404);` | **Modern `let-else` (Rust 1.65+)**: `let Some(user) = find_user(id) else { return Err(AppError::NotFound); };` |

---

## 3. Data Structures & OOP

| Modern Laravel (11/12) | Modern Rust (1.80+) | Architectural Insight |
|---|---|---|
| `readonly class UserDTO { public function __construct(public string $name) {} }` | `struct UserDto { pub name: String }` | Rust structs hold only pure data. |
| Property Hooks (PHP 8.4) / Getters | Methods on struct: `impl UserDto { pub fn full_name(&self) -> String { ... } }` | Behavior is implemented separately in `impl` blocks. |
| `$this->name` | `self.name` | Methods explicitly declare `&self` (read) or `&mut self` (write). |
| Static Constructor `User::create(...)` | `impl User { pub fn new(...) -> Self }` | Idiomatic constructor convention. |
| `interface HasUuid` | `trait HasUuid { fn uuid(&self) -> String; }` | Defines behavioral contracts. |
| PHP 8.1+ Enum | `enum Status: string { case Active; }` | `enum Status { Active, Suspended }` |
| Enum with Data (Algebraic Types) | ❌ Impossible without complex classes | `enum Event { Login(UserId), Error { code: u16, msg: String } }` |

---

## 4. Modern Web Architecture: Laravel 11/12 vs Axum 0.7+

| Modern Laravel 11/12 Component | Modern Rust (Axum 0.7+ & Tokio) Equivalent |
|---|---|
| **Entrypoint / App Setup** (`bootstrap/app.php`) | `src/main.rs` with `Router::new().with_state(...)` and `axum::serve(listener, app)` |
| **Routing** (`routes/api.php`) | `Router::new().route("/users", get(get_users).post(create_user))` |
| **Middleware Registration** (`withMiddleware` in `bootstrap/app.php`) | Tower Layers: `.layer(axum::middleware::from_fn(auth_guard))` |
| **Exception Handling** (`withExceptions` in `bootstrap/app.php`) | `IntoResponse` implementation on central `AppError` enum |
| **FormRequest / Validation** | `Json(payload): Json<CreateUserDto>` with `validator` crate |
| **Dependency Injection** | `State(state): State<Arc<AppState>>` |
| **Eloquent Model** (`casts(): array`) | Strongly typed struct with `sqlx::FromRow` and Serde |
| **Database Migrations** | `sqlx migrate run` (Embedded compile-time verified migrations) |
| **Artisan Command** | `clap` CLI crate (`#[derive(Parser)]`) |
| **Queues / Jobs** | Tokio MPSC channels (`tokio::sync::mpsc`) or Redis workers |
| **Modern Test Suite** (Pest 3) | Built-in `cargo test` (`#[test]`) with `assert_eq!` |
