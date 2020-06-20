use crate::mapping::error::TypeError;
use syn::{Type, TypePath, PathSegment, PathArguments, GenericArgument};

fn get_last_segment(path: &TypePath) ->  Result<&PathSegment, TypeError> {
    path.path.segments.iter().rev().next().ok_or(
        TypeError::new(path, "Cannot get the last segment of type")
    )
}

pub fn unwrap_type(value_type: &Type) -> Result<String, TypeError> {
    match value_type {
        Type::Path(path) => {
            let last_segment = get_last_segment(path)?;
            if !last_segment.arguments.is_empty() {
                Err(TypeError::new(value_type, "Generic is not supported in Entity"))
            } else {
                Ok(last_segment.ident.to_string())
            }
        },
        _ => Err(TypeError::new(value_type, "Expect to be a type path"))
    }
}

pub enum AssociationFieldType {
    Value(String),
    Collection(String)
}

#[allow(dead_code)]
impl AssociationFieldType {
    pub fn get_type(&self) -> String {
        match self {
            AssociationFieldType::Value(t) => t.clone(),
            AssociationFieldType::Collection(t) => t.clone()
        }
    }

    pub fn is_collection(&self) -> bool {
        match self {
            AssociationFieldType::Value(_) => false,
            AssociationFieldType::Collection(_) => true
        }
    }
}

pub fn unwrap_association_type(value_type: &Type) -> Result<AssociationFieldType, TypeError> {
    match value_type {
        Type::Path(path) => {
            let last_segment = get_last_segment(path)?;
            if last_segment.ident != "Collection" {
                if !last_segment.arguments.is_empty() {
                    Err(TypeError::new(value_type, "Generic is not supported in Association Field"))
                } else {
                    Ok(AssociationFieldType::Value(last_segment.ident.to_string()))
                }
            } else if last_segment.arguments.is_empty() {
                Err(TypeError::new(value_type, "Collection need 1 generic parameter"))
            } else {
                match &last_segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        let generic_type = args.args.iter().filter_map(
                            |item| {
                                match item {
                                    GenericArgument::Type(item_type) => Some(item_type),
                                    _ => None
                                }
                            }
                        ).fold(None,
                               |carry , item| {
                                   Some(carry.unwrap_or(item))
                               }
                        ).ok_or(TypeError::new(value_type, "Collection need 1 generic parameter."))?;

                        unwrap_type(generic_type).map(
                            |result| AssociationFieldType::Collection(result)
                        )
                    }
                    _ => Err(TypeError::new(value_type, "Only support generic in type argument"))
                }
            }
        },
        _ => Err(TypeError::new(value_type, "Expect to be a type path"))
    }
}

#[cfg(test)]
mod test {
    use super::{unwrap_type, unwrap_association_type};
    use syn::{Type, parse_quote};
    #[test]
    pub fn test_unwrap_type() {
        let type_a: Type = parse_quote! {
            std::collections::HashMap<String, i32>
        };
        let type_b: Type = parse_quote! {
            a::b::C
        };
        let type_c: Type = parse_quote! {
            A
        };

        assert_eq!(unwrap_type(&type_a).is_err(), true);
        assert_eq!(unwrap_type(&type_b).unwrap().as_str(), "C");
        assert_eq!(unwrap_type(&type_c).unwrap().as_str(), "A");
    }

    #[test]
    pub fn test_unwrap_association_type() {
        let type_a: Type = parse_quote! {
            model::model_a::B
        };
        let type_b: Type = parse_quote! {
            yuikino::Collection<model::model_a::B>
        };

        let type_c: Type = parse_quote! {
            Collection<B>
        };

        let type_d: Type = parse_quote! {
            B
        };

        let type_e: Type = parse_quote! {
            Collection<B<T>>
        };

        assert_eq!(unwrap_association_type(&type_a).unwrap().get_type(), "B");
        assert_eq!(unwrap_association_type(&type_b).unwrap().get_type(), "B");
        assert_eq!(unwrap_association_type(&type_c).unwrap().get_type(), "B");
        assert_eq!(unwrap_association_type(&type_d).unwrap().get_type(), "B");
        assert_eq!(unwrap_association_type(&type_e).is_err(), true);
    }
}