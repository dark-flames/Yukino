use crate::resolver::error::ResolveError;
use syn::{ItemUse, UseTree, TypePath};
use std::collections::HashMap;
use syn::__private::ToTokens;

pub type FullPath = String;
pub type TypeName = String;

pub struct TypeResolver {
    maps: HashMap<TypeName, FullPath>,
}

impl Default for TypeResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeResolver {
    pub fn new() -> TypeResolver {
        TypeResolver { maps: HashMap::new() }
    }

    pub fn append_use_item(&mut self, item: &ItemUse) -> Result<(), ResolveError> {
        let result = Self::resolve_use_tree(&item.tree)?;

        self.maps.extend(result.into_iter());

        Ok(())
    }

    fn resolve_use_tree(tree: &UseTree) -> Result<Vec<(TypeName, FullPath)>, ResolveError> {
        Ok(match tree {
            UseTree::Name(use_name) => {
                vec![(use_name.ident.to_string(), use_name.ident.to_string())]
            }
            UseTree::Rename(use_rename) => {
                vec![( use_rename.rename.to_string(), use_rename.ident.to_string())]
            }
            UseTree::Path(use_path) => {
                let current_segment = use_path.ident.to_string();
                let next = Self::resolve_use_tree(use_path.tree.as_ref()).map_err(|e| match e {
                    ResolveError::GlobInPathIsNotSupported(path) => {
                        ResolveError::GlobInPathIsNotSupported(format!(
                            "{}::{}",
                            current_segment, path
                        ))
                    }
                    others => others,
                })?;

                next.into_iter()
                    .map(|(full, name)| (name, format!("{}::{}", current_segment, full)))
                    .collect()
            }
            UseTree::Group(use_group) => use_group.items.iter().map(Self::resolve_use_tree).fold(
                Ok(vec![]),
                |carry, item_result| {
                    if let Ok(mut carry_vec) = carry {
                        if let Ok(mut item_vec) = item_result {
                            carry_vec.append(&mut item_vec);
                            Ok(carry_vec)
                        } else {
                            item_result
                        }
                    } else {
                        carry
                    }
                },
            )?,
            UseTree::Glob(_) => {
                return Err(ResolveError::GlobInPathIsNotSupported("*".to_string()))
            }
        })
    }

    pub fn get_full_path(&self, ty: &TypePath) -> String {
        let mut segments: Vec<_> = ty.path.segments.iter().map(
            |segment| segment.to_token_stream().to_string()
        ).collect();

        if let Some(full) = self.maps.get(segments.first().unwrap()) {
            segments[0] = full.clone();
        }

        segments.join("::")
    }
}
