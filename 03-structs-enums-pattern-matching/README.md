# Module 03: Structs, Enums & Modern Pattern Matching

In modern **Laravel 13** and **PHP 8.6**, data modeling has embraced **readonly classes**, **DTOs**, **asymmetric visibility** (`public private(set)`), and **property hooks**.

In Rust, data and behavior are cleanly separated by design:
- **`struct`**: Pure data container (like a modern readonly DTO with private fields and public getters).
- **`impl`**: Methods and business logic.
- **`enum`**: Tagged unions (each variant can carry distinct data).
- **`let-else` (Modern Rust)**: The ultimate guard clause replacing nested statements.

---

## 1. Structs as Modern DTOs (PHP 8.6 vs Rust)

### Modern PHP 8.6 Readonly DTO with Asymmetric Visibility & Property Hooks:
```php
<?php

readonly class CreateClientDTO {
    public function __construct(
        public string $clientCode,
        public string $boid,
        public float $initialDeposit = 0.0,
    ) {}

    // PHP 8.4+ / 8.6 Property Hook
    public string $displayLabel {
        get => "Client: {$this->clientCode} (Deposit: ৳{$this->initialDeposit})";
    }
}
```

### Modern Rust Struct:
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateClientDto {
    pub client_code: String,
    pub boid: String,
    pub initial_deposit: f64,
}

impl CreateClientDto {
    // Equivalent to a property hook getter
    pub fn display_label(&self) -> String {
        format!("Client: {} (Deposit: ৳{})", self.client_code, self.initial_deposit)
    }
}

fn main() {
    let dto = CreateClientDto {
        client_code: String::from("CL-1002"),
        boid: String::from("1201900012345678"),
        initial_deposit: 25_000.0,
    };

    println!("{}", dto.display_label());
}
```

---

## 2. Implementing Methods (`impl` Block)

Instead of mixing properties and methods inside a `class`, Rust declares data in `struct` and methods in `impl`:

```rust
pub struct ClientAccount {
    pub id: u64,
    pub client_code: String,
    pub balance: f64,
}

impl ClientAccount {
    // Associated function (Constructor convention - like ClientAccount::create())
    pub fn new(id: u64, client_code: String) -> Self {
        Self {
            id,
            client_code,
            balance: 0.0,
        }
    }

    // Read-only method (&self - like a getter)
    pub fn can_withdraw(&self, amount: f64) -> bool {
        self.balance >= amount
    }

    // State-mutating method (&mut self)
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}
```

---

## 3. Enums: Tagged Unions (Beyond PHP Enums)

In PHP, enums can only hold simple scalar values:
```php
<?php
enum OrderStatus: string {
    case Pending = 'pending';
    case Approved = 'approved';
}
```

In Rust, **each variant can carry completely different data payloads**:

```rust
#[derive(Debug)]
pub enum Notification {
    Email { to: String, subject: String },
    Sms { phone: String, message: String },
    Push { device_token: String, badge_count: u32 },
    SilentAudit,
}

fn send_notification(notif: Notification) {
    match notif {
        Notification::Email { to, subject } => {
            println!("Sending email to {to} with subject '{subject}'");
        }
        Notification::Sms { phone, message } => {
            println!("Sending SMS to {phone}: '{message}'");
        }
        Notification::Push { device_token, badge_count } => {
            println!("Push notification to {device_token} (Badge: {badge_count})");
        }
        Notification::SilentAudit => {
            println!("Recorded silent audit log");
        }
    }
}
```

---

## 4. Modern Guard Clauses: The `let-else` Statement

In modern Laravel 13 controllers, you frequently write guard clauses to return early if a record is not found or validation fails:

### Modern PHP 8.6 Guard Clause:
```php
<?php
$user = $this->userService->find($id);
if (!$user) {
    return response()->json(['error' => 'User not found'], 404);
}

// Continue using $user...
```

### Modern Rust `let-else` (Rust 1.65+):
Before `let-else`, Rust required nested `match` or `if let` blocks. Modern Rust provides **`let-else`**, which extracts a value or exits the current function:

```rust
fn handle_request(user_id: u64) -> Result<String, &'static str> {
    // If find_user returns Some(user), `user` is bound in the outer scope!
    // If it returns None, the else block MUST return, break, or panic!
    let Some(user) = find_user(user_id) else {
        return Err("User not found");
    };

    // `user` is directly available here as a normal, non-optional variable!
    Ok(format!("Welcome back, {}", user.name))
}
```

> **Why `let-else` is beloved:**  
> It eliminates rightward indentation drift (the "pyramid of doom") and matches the clean early-return style Laravel developers use every day.

---

## 5. Pattern Matching with `match`

Rust's `match` is:
1. **Exhaustive**: The compiler will not compile your code if you miss a single enum variant.
2. **An Expression**: It returns a value.

```rust
enum Role {
    Client,
    SupportAgent,
    SuperAdmin,
}

fn withdrawal_limit(role: &Role) -> f64 {
    match role {
        Role::Client => 500_000.0,
        Role::SupportAgent => 0.0,
        Role::SuperAdmin => 10_000_000.0,
    }
}
```

---

## 🛠️ Exercises: Module 03

1. **User Status Enum**:
   Create an enum `UserStatus` with variants:
   - `Unverified { email: String }`
   - `Active { last_login_timestamp: u64 }`
   - `Suspended { reason: String, until: u64 }`
2. **Guard Clause with `let-else`**:
   Write a function `process_withdrawal(account: Option<ClientAccount>, amount: f64) -> Result<f64, &'static str>` using `let-else` to ensure the account exists, then check if `account.can_withdraw(amount)`.
3. Check your work in [Comprehensive Exercise Solutions](../exercises-solutions/README.md).
