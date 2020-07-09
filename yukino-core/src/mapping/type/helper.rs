use crate::mapping::error::TypeError;
use std::iter::Rev;
use syn::punctuated::Iter;
use syn::{GenericArgument, PathArguments, PathSegment, Type, TypePath};

#[allow(dead_code)]
pub fn assert_type_path(value: &Type) -> Result<&TypePath, TypeError> {
    match value {
        Type::Path(path) => Ok(path),
        _ => Err(TypeError::new(value, "Expect a type path")),
    }
}

type Pair<T> = (T, T);
#[allow(dead_code)]
pub fn match_type_enum<'a>(
    type_a: &'a Type,
    type_b: &'a Type,
) -> Result<Pair<Rev<Iter<'a, PathSegment>>>, TypeError> {
    let type_path_a = assert_type_path(type_a)?;
    let type_path_b = assert_type_path(type_b)?;

    Ok((
        type_path_a.path.segments.iter().rev(),
        type_path_b.path.segments.iter().rev(),
    ))
}
#[allow(dead_code)]
pub fn cmp_type(type_a: &Type, type_b: &Type) -> Result<bool, TypeError> {
    let (mut a_iter, mut b_iter) = match_type_enum(type_a, type_b)?;

    loop {
        let a_item = a_iter.next();
        let b_item = b_iter.next();

        if a_item.is_none() || b_item.is_none() {
            break Ok(true);
        }

        let a_item = a_item.unwrap();
        let b_item = b_item.unwrap();

        if a_item.ident != b_item.ident {
            break Ok(false);
        }

        match &a_item.arguments {
            PathArguments::AngleBracketed(a_nested) => {
                if let PathArguments::AngleBracketed(b_nested) = &b_item.arguments {
                    let mut a_generic_iter = a_nested.args.iter();
                    let mut b_generic_iter = b_nested.args.iter();
                    match loop {
                        let a_generic_item = a_generic_iter.next();
                        let b_generic_item = b_generic_iter.next();

                        if a_generic_item.is_none() != b_generic_item.is_none() {
                            break Ok(false);
                        }
                        if a_generic_item.is_none() {
                            break Ok(true);
                        }

                        match cmp_generic_argument(a_generic_item.unwrap(), b_generic_item.unwrap())
                        {
                            Ok(false) => break Ok(false),
                            Ok(true) => (),
                            Err(e) => break Err(e),
                        };
                    } {
                        Ok(true) => (),
                        Ok(false) => break Ok(false),
                        Err(e) => break Err(e),
                    }
                }
            }
            PathArguments::None => {
                if PathArguments::None == b_item.arguments {
                    break Ok(true);
                }
            }
            _ => break Err(TypeError::new(type_b, "Unsupported type")),
        }
    }
}
#[allow(dead_code)]
pub fn cmp_generic_argument(
    param_a: &GenericArgument,
    param_b: &GenericArgument,
) -> Result<bool, TypeError> {
    match param_a {
        GenericArgument::Type(type_a) => {
            if let GenericArgument::Type(type_b) = param_b {
                cmp_type(type_a, type_b)
            } else {
                Ok(false)
            }
        }
        _ => {
            if param_a == param_b {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cmp_type;
    use syn::{parse_quote, Type};

    #[test]
    pub fn test_cmp_type() {
        let type_a: Type = parse_quote! {
            std::collections::HashMap<String, i32>
        };
        let type_b: Type = parse_quote! {
            std::collections::HashMap<String, i64>
        };
        let type_c: Type = parse_quote! {
            HashMap<String, i32>
        };

        assert_eq!(cmp_type(&type_a, &type_c).unwrap(), true);
        assert_eq!(cmp_type(&type_a, &type_b).unwrap(), false);
    }
}
