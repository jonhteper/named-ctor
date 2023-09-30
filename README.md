# named-ctor

Procedural macro to generate an alternative of named parameters with native Rust.

## Named parameters alternative
Rust don't has support for named parameters, but is possible to use a simulation with data structures.
This alternative is special easy to implement in Rust tanks to `From` trait.

For example:

```rust
pub struct User {
    id: u8,
    name: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
}

impl User {
    // ...
}

pub struct UserValues {
    id: u8,
    name: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
}

impl From<UserValues> for User {
    fn from(aux: UserValues) -> Self {
        Self {
            id: aux.id,
            name: aux.name,
            email: aux.email,
            password: aux.password,
            is_active: aux.is_active,
            is_admin: aux.is_admin,
        }
    }
}

pub fn main() {
    let user: User = User::from(UserValues {
        id: 0,
        email: "john@doe.com".to_string(),
        name: "John Doe".to_string(),
        is_active: true,
        password: "1234".to_string(),
        is_admin: false,
    });
}
```

Whats the problem? First, too boilerplate. Second, its not easy to maintain, if `User` is modified, is absolutely necessary modify `UserValues`. 

## Using `NamedCtor` macro

The behavior is the same as last example, but now the macro is the responsable to create both `UserValues` and `From` implementation.

```rust
use named_ctor::NamedCtor;

#[derive(NamedCtor)]
pub struct User {
    id: u8,
    name: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
}

impl User {
    // ...
}

pub fn main() {
    let user: User = User::from(UserValues {
        id: 0,
        email: "john@doe.com".to_string(),
        name: "John Doe".to_string(),
        is_active: true,
        password: "1234".to_string(),
        is_admin: false,
    });
}
```
### Macro attributes

Is possible to use a custom name for the axiliar struct, and change the
constructor function:

```rust
use named_ctor::NamedCtor;
use core::fmt::Display;
#[derive(NamedCtor)]
#[named_ctor(name = "TaskInitValues", constructor = "new")]
pub struct Task<'a, T>
where
    T: Display
{
    id: T,
    name: &'a str,
}
let user: Task<&str> = Task::new(TaskInitValues {
    id: "example.id",
    name: "Example",
});
```

**WARNING**: Generics support only via where clause