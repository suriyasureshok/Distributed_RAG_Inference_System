# 🦀 Rust Docstring Formatting Guide (Production Standard)

## 🔹 0. Global Rules

* Use `///` for public API docs
* Use `//!` for module-level / crate-level docs
* Write in **Markdown**
* First line = **short summary (one sentence)**
* Leave **one blank line** after summary
* Use **imperative voice** ("Returns...", "Creates...")
* Always document:

  * Public items
  * Non-trivial private items (optional but recommended)

---

# 🔹 1. Crate-Level Docs

````rust
//! # Crate Name
//!
//! One-line description of the crate.
//!
//! ## Overview
//! Brief explanation of purpose.
//!
//! ## Features
//! - Feature A
//! - Feature B
//!
//! ## Example
//! ```rust
//! use crate_name::SomeStruct;
//!
//! let x = SomeStruct::new();
//! ```
//!
//! ## Safety
//! Mention if unsafe code is used.
//!
//! ## Errors
//! Describe global error behavior if relevant.
````

---

# 🔹 2. Module Docs

````rust
//! # Module Name
//!
//! Short description.
//!
//! ## Responsibilities
//! - Handles X
//! - Provides Y
//!
//! ## Design
//! High-level architecture decisions.
//!
//! ## Examples
//! ```rust
//! use crate::module::Type;
//! ```
//!
//! ## Invariants
//! - Must always hold true
````

---

# 🔹 3. Structs

````rust
/// Represents a user in the system.
///
/// ## Fields
/// - `id`: Unique identifier
/// - `name`: User name
///
/// ## Invariants
/// - `id` must be non-zero
///
/// ## Examples
/// ```rust
/// let user = User::new(1, "Alice".into());
/// ```
pub struct User {
    pub id: u64,
    pub name: String,
}
````

---

# 🔹 4. Enums

````rust
/// Represents possible states of a task.
///
/// ## Variants
/// - `Pending`: Task not started
/// - `Running`: Task in progress
/// - `Completed`: Task finished
///
/// ## Examples
/// ```rust
/// let state = TaskState::Pending;
/// ```
pub enum TaskState {
    Pending,
    Running,
    Completed,
}
````

---

# 🔹 5. Traits

````rust
/// Defines behavior for a storage backend.
///
/// ## Required Methods
/// - `save`: Persist data
/// - `load`: Retrieve data
///
/// ## Provided Methods
/// - `exists`: Check presence
///
/// ## Errors
/// Implementations must define error behavior.
///
/// ## Examples
/// ```rust
/// struct MemoryStore;
/// ```
pub trait Storage {
    fn save(&self);
    fn load(&self);
}
````

---

# 🔹 6. Trait Implementations

```rust
/// In-memory implementation of `Storage`.
///
/// ## Performance
/// O(1) operations.
///
/// ## Thread Safety
/// Not thread-safe.
impl Storage for MemoryStore {
    fn save(&self) {}
    fn load(&self) {}
}
```

---

# 🔹 7. Impl Blocks

```rust
/// Methods for `User`.
///
/// ## Design Notes
/// - Immutable ID
/// - Owned name
impl User {
```

---

# 🔹 8. Functions

````rust
/// Creates a new user.
///
/// ## Arguments
/// - `id`: Unique identifier
/// - `name`: User name
///
/// ## Returns
/// A new `User` instance.
///
/// ## Errors
/// Returns error if `id == 0`.
///
/// ## Panics
/// Panics if name is empty.
///
/// ## Examples
/// ```rust
/// let user = User::new(1, "Alice".into());
/// ```
pub fn new(id: u64, name: String) -> Self {
````

---

# 🔹 9. Methods

````rust
/// Updates the user's name.
///
/// ## Arguments
/// - `name`: New name
///
/// ## Panics
/// Panics if name is empty.
///
/// ## Examples
/// ```rust
/// user.update_name("Bob".into());
/// ```
pub fn update_name(&mut self, name: String) {
````

---

# 🔹 10. Inline / Private Functions

```rust
/// Validates input data.
///
/// Internal helper. Not exposed publicly.
///
/// ## Returns
/// `true` if valid.
fn validate(input: &str) -> bool {
```

---

# 🔹 11. Constants

```rust
/// Maximum allowed retries.
pub const MAX_RETRIES: u32 = 3;
```

---

# 🔹 12. Macros

````rust
/// Logs a formatted message.
///
/// ## Examples
/// ```rust
/// log_msg!("Hello {}", "world");
/// ```
#[macro_export]
macro_rules! log_msg {
````

---

# 🔹 13. Unsafe Code

```rust
/// Performs unchecked memory access.
///
/// ## Safety
/// Caller must ensure:
/// - Pointer is valid
/// - No aliasing occurs
///
/// ## Undefined Behavior
/// Violating safety rules leads to UB.
unsafe fn dangerous(ptr: *const i32) {
```

---

# 🔹 14. Error Types

````rust
/// Errors that can occur during parsing.
///
/// ## Variants
/// - `InvalidFormat`: Input is malformed
/// - `Overflow`: Value too large
///
/// ## Examples
/// ```rust
/// let err = ParseError::InvalidFormat;
/// ```
pub enum ParseError {
    InvalidFormat,
    Overflow,
}
````

---

# 🔹 15. Generics & Lifetimes

````rust
/// Wraps a reference with lifetime `'a`.
///
/// ## Type Parameters
/// - `'a`: Lifetime of the reference
///
/// ## Examples
/// ```rust
/// let w = Wrapper { value: &x };
/// ```
pub struct Wrapper<'a, T> {
    pub value: &'a T,
}
````

---

# 🔹 16. Async Functions

````rust
/// Fetches data asynchronously.
///
/// ## Errors
/// Returns network errors.
///
/// ## Examples
/// ```rust
/// let data = fetch().await?;
/// ```
pub async fn fetch() -> Result<Data, Error> {
````

---

# 🔹 17. Sections Standard (Use Consistently)

Use these headers where applicable:

* `## Arguments`
* `## Returns`
* `## Errors`
* `## Panics`
* `## Safety`
* `## Examples`
* `## Invariants`
* `## Performance`
* `## Thread Safety`
* `## Type Parameters`

---

# 🔹 18. Formatting Rules

* Backticks for:

  * Types: `User`
  * Params: `id`
  * Functions: `new()`
* Code blocks must compile
* Avoid long paragraphs
* Prefer bullet points

---

# 🔹 19. Anti-Patterns (Don’t Do This)

❌ Repeating function name
❌ Writing essays
❌ Missing examples
❌ Ignoring panics/errors
❌ Documenting obvious code

---

# 🔹 20. Production Enforcement

* Use:

  * `cargo doc --document-private-items`
  * `rustdoc` warnings
  * Clippy: `missing_docs`
* CI rule:

  * Fail if public items lack docs

---

If you actually follow this strictly, your Rust docs won’t look like a student project—they’ll look like something people trust in prod.
