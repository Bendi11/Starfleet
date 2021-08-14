use proc_macro::{TokenStream, TokenTree};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;


lazy_static! {
    /// A set of all used hash values, used to detect collisions at compile time
    static ref HASHES: Arc<RwLock<HashSet<usize>>> = Arc::new(RwLock::new(HashSet::new()));
}

/// Register this as a component type for serialization and deserialization
/// ## Example
/// ```rust,no_run
/// #[component]
/// pub struct Health {
///     f32 health
/// }
/// ```
///
/// If a hash collision ever does occur, the problem will be detected and compilation will fail, guranteeing no hard to detect 
/// bugs with serialization occur. If the problem does arise, use the `name = val` syntax to specify a different name for serializing and
/// deserializing the component
/// 
/// ```rust,no_run
/// #[component(name = "Health1")]
/// pub struct Health {
///     f32 health
/// }
/// ```
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let original = item.clone();
    let mut toks = item.into_iter();
    let name = match toks.next() {
        TokenTree::Ident(ident) => {

        },
        other => 
    }
    
    original
}