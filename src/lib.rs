use proc_macro::TokenStream;

mod named_ctor;

/// Auto create constructor via [From] trait or using `new` method.
/// **WARNING**: Generics support only via where clause
///
/// ## Examples
///
/// By default the macro use the [From] trait, and the auxiliar struct
/// has the same name with prefix "_"
/// ```
/// use named_ctor::NamedCtor;
///
/// #[derive(NamedCtor)]
/// pub struct User {
///     id: u8,
///     name: String,
/// }
///
/// let user: User = User::from(_User{
///     id: 0,
///     name: "John Doe".to_string()
/// });
/// ```
///
///
/// Is possible to use a custom name for the axiliar struct, and define
/// the constructor type:
/// ```
/// use named_ctor::NamedCtor;
/// use core::fmt::Display;
///
/// #[derive(NamedCtor)]
/// #[named_ctor(name = "TaskInitValues", constructor = "new")]
/// pub struct Task<'a, T>
/// where
///     T: Display
/// {
///     id: T,
///     name: &'a str,
/// }
///
/// let user: Task<&str> = Task::new(TaskInitValues {
///     id: "example.id",
///     name: "Example",
/// });
/// ```
///
#[proc_macro_derive(NamedCtor, attributes(named_ctor))]
pub fn derive_struct_values(input: TokenStream) -> TokenStream {
    named_ctor::struct_values_macro(input)
}
