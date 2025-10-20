use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit};

struct FromEnvConfig {
    prefix: Option<String>,
    word_separator: Option<String>,
}

impl FromEnvConfig {
    fn from_attributes(attrs: &[syn::Attribute]) -> Self {
        let mut prefix = None;
        let mut word_separator = None;

        for attr in attrs {
            if !attr.path().is_ident("from_env") {
                continue;
            }

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let value = meta.value()?;
                    let s: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = s {
                        prefix = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("word_separator") {
                    let value = meta.value()?;
                    let s: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = s {
                        word_separator = Some(lit_str.value());
                    }
                }
                Ok(())
            });
        }

        FromEnvConfig {
            prefix,
            word_separator,
        }
    }
}

fn to_env_case(s: &str, separator: Option<&str>) -> String {
    let separator = separator.unwrap_or("");

    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && prev_is_lower && !separator.is_empty() {
                result.push_str(separator);
            }
            result.push(ch.to_ascii_uppercase());
            prev_is_lower = false;
        } else {
            result.push(ch.to_ascii_uppercase());
            prev_is_lower = ch.is_lowercase();
        }
    }

    result
}

#[proc_macro_derive(FromEnv, attributes(from_env))]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let config = FromEnvConfig::from_attributes(&input.attrs);

    let struct_name_env = to_env_case(&name.to_string(), config.word_separator.as_deref());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromEnv only supports structs with named fields"),
        },
        _ => panic!("FromEnv can only be derived for structs"),
    };

    let mut from_env_assignments = Vec::new();
    let mut load_from_env_assignments = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_name_str = field_name.to_string();
        let field_name_env = field_name_str.to_uppercase();

        let env_var_name = if let Some(prefix) = &config.prefix {
            format!("{}{}_{}", prefix, struct_name_env, field_name_env)
        } else {
            format!("{}_{}", struct_name_env, field_name_env)
        };

        let parse_expr = generate_parse_expr(field_type, &env_var_name);

        from_env_assignments.push(quote! {
            #field_name: #parse_expr,
        });

        load_from_env_assignments.push(quote! {
            self.#field_name = #parse_expr;
        });
    }

    let expanded = quote! {
        impl from_env::FromEnvTrait for #name {
            fn from_env() -> Result<Self, from_env::FromEnvError> {
                Ok(Self {
                    #(#from_env_assignments)*
                })
            }

            fn load_from_env(&mut self) -> Result<(), from_env::FromEnvError> {
                #(#load_from_env_assignments)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_inner_type<'a>(ty: &'a syn::Type, wrapper: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == wrapper {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

fn generate_parse_expr(field_type: &syn::Type, env_var_name: &str) -> proc_macro2::TokenStream {
    // Check for Option<T>
    if let Some(inner_type) = extract_inner_type(field_type, "Option") {
        let inner_parse = generate_required_parse_expr(inner_type, env_var_name);
        return quote! {
            match std::env::var(#env_var_name) {
                Ok(_) => Some(#inner_parse),
                Err(_) => None,
            }
        };
    }

    // Check for Vec<T>
    if let Some(inner_type) = extract_inner_type(field_type, "Vec") {
        return generate_vec_parse_expr(inner_type, env_var_name);
    }

    // Regular required field
    generate_required_parse_expr(field_type, env_var_name)
}

fn generate_required_parse_expr(field_type: &syn::Type, env_var_name: &str) -> proc_macro2::TokenStream {
    let type_str = quote!(#field_type).to_string();
    let type_str = type_str.replace(" ", "");

    match type_str.as_str() {
        "String" => {
            quote! {
                std::env::var(#env_var_name)
                    .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?
            }
        }
        "bool" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_bool(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "char" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_char(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    value.parse::<#field_type>()
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: from_env::ParseError::ParseInt(e),
                        })?
                }
            }
        }
        "f32" | "f64" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    value.parse::<#field_type>()
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: from_env::ParseError::ParseFloat(e),
                        })?
                }
            }
        }
        _ => {
            // For any other type, try to use FromStr
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    value.parse::<#field_type>()
                        .map_err(|_| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: from_env::ParseError::InvalidUtf8,
                        })?
                }
            }
        }
    }
}

fn generate_vec_parse_expr(inner_type: &syn::Type, env_var_name: &str) -> proc_macro2::TokenStream {
    let type_str = quote!(#inner_type).to_string();
    let type_str = type_str.replace(" ", "");

    match type_str.as_str() {
        "String" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_string(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "bool" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_bool(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "char" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_char(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_int::<#inner_type>(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        "f32" | "f64" => {
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_float::<#inner_type>(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
        _ => {
            // For any other type, try generic vector parsing
            quote! {
                {
                    let value = std::env::var(#env_var_name)
                        .map_err(|_| from_env::FromEnvError::MissingVariable(#env_var_name.to_string()))?;
                    from_env::parse_vec_int::<#inner_type>(&value)
                        .map_err(|e| from_env::FromEnvError::ParseError {
                            var: #env_var_name.to_string(),
                            source: e,
                        })?
                }
            }
        }
    }
}
