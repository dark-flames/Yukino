use crate::association::AssociatedEntityFieldResolverSeed;
use crate::resolver::entity_resolver_passes::{
    EntityImplementResolverPass, EntityProxyResolverPass, EntityStructResolverPass,
};
use crate::resolver::error::ResolveError;
use crate::resolver::field_resolver_seeds::{
    CollectionFieldResolverSeed, NumericFieldResolverSeed, StringFieldResolverSeed,
};
use crate::resolver::{
    EntityResolverPass, EntityResolverPassBox, FieldResolverSeed, FieldResolverSeedBox,
    ImmutableSchemaResolver, SchemaResolver, TypePathResolver,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;
use syn::{parse_file, Error as SynError, Item};

pub struct FileResolver {
    schema_resolver: SchemaResolver,
    type_path_resolver: TypePathResolver,
    schema_file: File,
}

impl FileResolver {
    pub fn new(
        mut customized_seeds: Vec<FieldResolverSeedBox>,
        mut customized_entity_resolver_pass: Vec<EntityResolverPassBox>,
        schema_file_path: String,
    ) -> Result<Self, ResolveError> {
        let schema_file = File::open(schema_file_path).map_err(ResolveError::IOError)?;

        let mut default_seeds: Vec<FieldResolverSeedBox> = vec![
            Box::new(NumericFieldResolverSeed::new()),
            Box::new(CollectionFieldResolverSeed::new()),
            Box::new(StringFieldResolverSeed::new()),
            Box::new(AssociatedEntityFieldResolverSeed::new()),
        ];

        let mut default_passes: Vec<EntityResolverPassBox> = vec![
            Box::new(EntityStructResolverPass::new()),
            Box::new(EntityImplementResolverPass::new()),
            Box::new(EntityProxyResolverPass::new()),
        ];

        customized_seeds.append(&mut default_seeds);
        customized_entity_resolver_pass.append(&mut default_passes);

        Ok(FileResolver {
            schema_resolver: SchemaResolver::new(customized_seeds, customized_entity_resolver_pass),
            type_path_resolver: Default::default(),
            schema_file,
        })
    }

    pub fn resolve(mut self) -> Result<AchievedFileResolver, SynError> {
        let mut content = String::new();
        self.schema_file
            .read_to_string(&mut content)
            .map_err(|e| ResolveError::IOError(e).into_syn_error(""))?;

        let syntax = parse_file(content.as_str())
            .map_err(|e| ResolveError::ParseError(e.to_string()).into_syn_error(""))?;

        for use_item in syntax.items.iter().filter_map(|item| match item {
            Item::Use(item_use) => Some(item_use),
            _ => None,
        }) {
            self.type_path_resolver
                .append_use_item(use_item)
                .map_err(|e| e.into_syn_error(use_item))?;
        }

        for item in syntax.items {
            match item {
                Item::Struct(item_struct) => {
                    self.schema_resolver
                        .parse(item_struct, &self.type_path_resolver)?;
                }
                Item::Use(_) => {}
                _ => return Err(ResolveError::UnsupportedSyntaxBlock.into_syn_error(item)),
            }
        }

        Ok(AchievedFileResolver {
            schema_resolver: self.schema_resolver.achieve(&self.type_path_resolver)?,
        })
    }
}

pub struct AchievedFileResolver {
    schema_resolver: ImmutableSchemaResolver,
}

impl AchievedFileResolver {
    pub fn get_result(&self) -> TokenStream {
        let result = self.schema_resolver.get_implements();

        quote! {
            #![allow(unknown_lints)]
            #result
        }
    }
}
