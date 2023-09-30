use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields, Lit, Meta, Token};

#[derive(Debug, Default)]
enum ContructorType {
    New,

    #[default]
    From,
}

impl From<String> for ContructorType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "new" => ContructorType::New,
            "from" => ContructorType::From,
            _ => panic!("invalid attribute value: `{s}`; use `new` or `from` instead"),
        }
    }
}

#[derive(Default, Debug)]
struct StructValuesAttr {
    name: Option<String>,
    constructor_type: ContructorType,
}

impl StructValuesAttr {
    fn name_as_ident(&self, original_struct_ident: &Ident) -> Ident {
        let name = self
            .name
            .clone()
            .unwrap_or(format!("_{}", original_struct_ident));

        Ident::new(&name, original_struct_ident.span())
    }

    fn extract_string(meta: Meta, attr_name: &str) -> String {
        if let Meta::NameValue(name_val) = meta {
            if let Lit::Str(name) = &name_val.lit {
                return name.value();
            }
        }

        panic!("invalid `{attr_name}` attribute");
    }

    fn new(attrs: &[syn::Attribute]) -> Self {
        let mut struct_values_attr = Self::default();
        for attr in attrs {
            if !attr.path.is_ident("named_ctor") {
                continue;
            }

            let attributes = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();

            for meta in attributes {
                if let Some(ident) = meta.path().get_ident() {
                    match ident.to_string().as_str() {
                        "name" => {
                            struct_values_attr.name = Some(Self::extract_string(meta, "name"));
                        }
                        "constructor" => {
                            let constructor_type = Self::extract_string(meta, "constructor");
                            struct_values_attr.constructor_type = constructor_type.into();
                        }
                        _ => panic!("unexpected attribute, use `name` or `constructor`"),
                    }
                } else {
                    panic!("unexpected attribute, use `name` or `constructor`");
                }
            }
        }

        struct_values_attr
    }
}

pub fn struct_values_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let macro_attrs = StructValuesAttr::new(&input.attrs);
    let original_ident = input.ident;
    let struct_identifier = macro_attrs.name_as_ident(&original_ident);

    let lifetimes: Vec<_> = input.generics.lifetimes().collect();
    let type_params: Vec<_> = input.generics.type_params().collect();
    let where_clause = &input.generics.where_clause;

    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        panic!("NamedCtor only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("NamedCtor only supports structs with named fields");
    };

    let mut struct_fields = Vec::new();
    let mut pair_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        struct_fields.push(quote! {
            pub #field_name: #field_type,
        });

        pair_fields.push(quote! {
            #field_name: values.#field_name,
        });
    }

    let generics = quote! {
        <#(#lifetimes,)* #(#type_params),*>
    };

    let constructor_impl = match macro_attrs.constructor_type {
        ContructorType::New => {
            quote! {
                impl #generics #original_ident #generics
                #where_clause {
                    fn new(values: #struct_identifier #generics) -> Self {
                        Self {
                            #(#pair_fields)*
                        }
                    }
                }

            }
        }
        ContructorType::From => {
            quote! {
                impl #generics From<#struct_identifier #generics> for #original_ident #generics
                 #where_clause {
                    fn from(values: #struct_identifier #generics) -> Self {
                        Self {
                            #(#pair_fields)*
                        }
                    }
                }

            }
        }
    };

    let expanded = quote! {
        pub struct #struct_identifier #generics #where_clause {
            #(#struct_fields)*
        }

        #constructor_impl
    };

    TokenStream::from(expanded)
}
