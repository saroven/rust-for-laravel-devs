# Module 03: Structs, Enums & Pattern Matching

In PHP and Laravel, you use `class` for everything: Models, DTOs, Services, Value Objects, and Controllers.

In Rust, data and behavior are cleanly decoupled:
- **`struct`**: Holds data (like a clean DTO or database row).
- **`impl`**: Defines methods and behavior for that struct.
- **`enum`**: Represents one of several variants (vastly more powerful than PHP 8.1 enums).
- **`match`**: Robust, compiler-enforced pattern matching.

---

## 1. Structs: Data Containers

### PHP DTO:
```php
<?php
readonly class ClientAccount {
    public function __construct(
        public int $id,
        public string $clientCode,
        public float $balance = 0.0,
    ) {}
}
```

### Rust Struct:
```rust
struct ClientAccount {
    id: u64,
    client_code: String,
    balance: f64,
}

fn main() {
    // Instantiation
    let mut account = ClientAccount {
        id: 101,
        client_code: String::from("CL-992"),
        balance: 15000.50,
    };

    account.balance += 500.0;
    println!("Client {} has ৳{}", account.client_code, account.balance);
}
```

### Struct Field Init Shorthand:
Just like modern JS and PHP 8 named arguments, if variable names match field names:
```rust
fn create_client(id: u64, client_code: String) -> ClientAccount {
    ClientAccount {
        id,
        client_code,
        balance: 0.0,
    }
}
```

---

## 2. Implementing Methods (`impl` Block)

Instead of declaring methods inside the class body, Rust uses `impl`:

```rust
struct ClientAccount {
    id: u64,
    client_code: String,
    balance: f64,
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

    // Method taking immutable reference (&self - read-only)
    pub fn can_withdraw(&self, amount: f64) -> bool {
        self.balance >= amount
    }

    // Method taking mutable reference (&mut self - modifies state)
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}

fn main() {
    let mut acc = ClientAccount::new(1, String::from("CL-001"));
    acc.deposit(2500.0);
    println!("Can withdraw 1000? {}", acc.can_withdraw(1000.0));
}
```

Notice:
- `&self`: Like `$this`, but read-only.
- `&mut self`: Allows mutating `$this`.
- `self`: Consumes the instance (moves ownership, rare for normal methods).

---

## 3. Tuple Structs & Newtype Pattern

Sometimes you want strong typing without naming every field. This is called the **Newtype pattern**:

```rust
// Strongly-typed IDs (Prevents accidentally passing an OrderId to a ClientId parameter!)
struct ClientId(u64);
struct OrderId(u64);

fn cancel_order(client: ClientId, order: OrderId) {
    println!("Cancelling order {} for client {}", order.0, client.0);
}
```

In PHP, both are integers and swapping them is a silent bug. In Rust, passing `order` where `client` is expected will fail at compile time!

---

## 4. Enums: Algebraic Data Types (Tagged Unions)

PHP 8.1 introduced Enums, but they are limited to simple scalars:

```php
<?php
// PHP 8.1
enum OrderStatus: string {
    case Pending = 'pending';
    case Approved = 'approved';
    case Rejected = 'rejected';
}
```

In Rust, **each enum variant can carry its own distinct data payload**:

```rust
enum PaymentMethod {
    Cash,
    BankTransfer { bank_name: String, account_number: String },
    Bkash { phone_number: String, transaction_id: String },
    CreditCard { card_number: String, expiry: String },
}

fn process_payment(method: PaymentMethod) {
    match method {
        PaymentMethod::Cash => {
            println!("Cash received at branch counter.");
        }
        PaymentMethod::BankTransfer { bank_name, account_number } => {
            println!("Bank transfer to {bank_name} (A/C: {account_number})");
        }
        PaymentMethod::Bkash { phone_number, transaction_id } => {
            println!("bKash payment from {phone_number}, TxnID: {transaction_id}");
        }
        PaymentMethod::CreditCard { card_number, .. } => {
            println!("Card charged: ending in {}", &card_number[card_number.len()-4..]);
        }
    }
}
```

> **Why this changes everything:**  
> In PHP/Laravel, you would have to model this with 4 classes extending a base `PaymentMethod` class or a polymorphic database relationship. In Rust, an enum elegantly captures all possible states in a single, type-safe type.

---

## 5. Pattern Matching with `match`

Rust's `match` is:
1. **Exhaustive**: If you forget to handle one enum variant, the compiler rejects your code!
2. **An Expression**: It returns a value.

```rust
enum Role {
    Client,
    SupportAgent,
    SuperAdmin,
}

fn max_daily_withdrawal_limit(role: Role) -> f64 {
    match role {
        Role::Client => 500_000.0,
        Role::SupportAgent => 0.0, // Support agents cannot withdraw
        Role::SuperAdmin => 10_000_000.0,
    }
}
```

### The `_` Wildcard (Default):
```rust
match code {
    200 => println!("OK"),
    404 => println!("Not Found"),
    500..=599 => println!("Server Error"),
    _ => println!("Other status code"),
}
```

---

## 6. Concise Matching: `if let`

When you only care about matching **one** variant and ignoring the rest:

```rust
let payment = PaymentMethod::Bkash { 
    phone_number: String::from("01700000000"), 
    transaction_id: String::from("TRX8891") 
};

// Instead of writing a full match with _ => ()
if let PaymentMethod::Bkash { transaction_id, .. } = payment {
    println!("Audit log bKash Txn: {transaction_id}");
}
```

---

## 🛠️ Exercises: Module 03

1. **User Status Enum**:
   Create an enum `UserStatus` with variants:
   - `Unverified { email: String }`
   - `Active { last_login_timestamp: u64 }`
   - `Suspended { reason: String, until: u64 }`
2. **Display Function**:
   Write a function `format_status(status: &UserStatus) -> String` using `match` to return human-friendly descriptions for each variant.
3. **Struct with `impl`**:
   Create a `Portfolio` struct containing `client_id: u64`, `stock_value: f64`, and `cash_balance: f64`. Implement a method `total_asset_value(&self) -> f64` that sums both values.
