use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{
    parse::Parser, parse_macro_input, Item, ItemEnum, ItemFn, ItemStruct, ItemType, ItemUnion,
    Token,
};

/// From [here](http://www.isthe.com/chongo/tech/comp/fnv/)
const FNV_OFFSET_BASIS: u64 = 14695981039346656037u64;
/// From [here](http://www.isthe.com/chongo/tech/comp/fnv/)
const FNV_PRIME: u64 = 1099511628211;

/// Hash a byte array with the fnv1a-64 hashing algorithm
#[inline]
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in bytes {
        hash = (hash ^ (*byte as u64)).wrapping_mul(FNV_PRIME);
    }
    hash
}

lazy_static! {
    /// A set of all used hash values, used to detect collisions at compile time
    static ref HASHES: Arc<Mutex<HashMap<u64, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Attributes given as arguments to a procedural macro
struct Attrs(pub HashMap<String, String>);

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut map = HashMap::new();
        loop {
            if input.is_empty() {
                break;
            }
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let string: syn::LitStr = input.parse()?;
            match input.peek(Token![,]) {
                true => {
                    let _: Token![,] = input.parse()?;
                }
                false => {
                    if !input.is_empty() {
                        return Err(syn::Error::new(
                            input.span(),
                            "Expected a comma between procedural macro arguments",
                        ));
                    }
                }
            }
            map.insert(ident.to_string(), string.value());
        }
        Ok(Self(map))
    }
}

/// Register this as a component type for serialization and deserialization
/// ## Example
/// ```ignore
/// #[component]
/// pub struct Health {
///     health: f32
/// }
/// ```
///
/// If a hash collision ever does occur, the problem will be detected and compilation will fail, guranteeing no hard to detect
/// bugs with serialization occur. If the problem does arise, use the `name = val` syntax to specify a different name for serializing and
/// deserializing the component
///
/// ```ignore
/// #[component(name = "Health1")]
/// pub struct Health {
///     health: f32
/// }
/// ```
#[proc_macro_attribute]
pub fn component(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attrs);
    let def: TokenStream = item.clone().into();
    let parsed = parse_macro_input!(def as Item);
    let name = match parsed {
        Item::Enum(ItemEnum { ident, .. })
        | Item::Struct(ItemStruct { ident, .. })
        | Item::Type(ItemType { ident, .. })
        | Item::Union(ItemUnion { ident, .. }) => ident,
        other => {
            return quote_spanned! {
                other.span() =>
                compile_error!("Expected type declaration below component attribute macro");
            }
            .into()
        }
    };

    name.span().start();

    //Get the name to hash for identifier, to fix hash collisions
    let hash_name = match attrs.0.get("name") {
        Some(name) => name.clone(),
        None => name.to_string(),
    };
    let hash = fnv1a(hash_name.as_bytes()); //Get the hash of the chosen identifier for component ID

    let mut hashes = HASHES.lock().unwrap(); //Get the collection of hashes to check for a collision
    match hashes.get(&hash) {
        Some(other) => {
            let errmsg = format!(
                "Hash collision occurred: {} collides with {}",
                hash_name, other
            );
            return quote_spanned! {
                name.span().into() =>
                compile_error!( #errmsg );
            }
            .into();
        }
        None => {
            hashes.insert(hash, hash_name);
        }
    }

    let static_name = quote::format_ident!("_{}", hash);
    let register_fn_name = quote::format_ident!("_{}_register", hash);

    let component_impl = quote! {
        fn #register_fn_name (registry: &mut ::legion::serialize::Registry<u64>) {
            registry.register::<#name>(#hash);
        }

        #[cfg(use_inventory)]
        ::inventory::submit! {
            crate::register::RegistrarFunction( #register_fn_name )
        }

        #[cfg(use_linkme)]
        #[::linkme::distributed_slice(crate::register::COMPONENT_HASHES)]
        static #static_name: fn(&mut ::legion::serialize::Registry<u64>) = #register_fn_name;
    };

    item.extend(TokenStream::from(component_impl));
    item
}

/// Register this system to run at the given event or events
/// Requires an argument for the event name
#[proc_macro_attribute]
pub fn on_event(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let def: TokenStream = item.clone().into();
    let def: ItemFn = parse_macro_input!(def as ItemFn);
    let name = def.sig.ident;

    //Parse the list of events for this system
    let parser = syn::punctuated::Punctuated::<syn::Ident, Token![,]>::parse_separated_nonempty;
    let list = parser.parse(attr).unwrap();

    for event in list.iter().map(|ident| ident.to_string()) {
        let register_fn_name = quote::format_ident!("_{}_register", name);
        let system_fn_name = quote::format_ident!("{}_system", name);
        let event_name = quote::format_ident!("{}", event.to_lowercase());
        let static_name = quote::format_ident!("_{}_REGISTRAR", name.to_string().to_uppercase());

        let system_impl = quote! {
            fn #register_fn_name (schedules: &mut crate::register::SchedulesBuilder) {
                schedules.#event_name.add_system( #system_fn_name() );
            }

            #[cfg(use_inventory)]
            ::inventory::submit! {
                crate::register::SystemRegistrarFunction( #register_fn_name )
            }

            #[cfg(use_linkme)]
            #[::linkme::distributed_slice(crate::register::SYSTEM_REGISTRARS)]
            static #static_name: fn(&mut crate::register::SchedulesBuilder) = #register_fn_name;
        };
        let toks: TokenStream = system_impl.into();
        item.extend(toks); //Add the definition
    }

    item
}
