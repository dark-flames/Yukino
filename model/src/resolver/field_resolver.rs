use crate::resolver::{EntityPath, FieldPath};

pub enum FieldResolverStatus {
    Seed,
    WaitingForFields(Vec<FieldPath>),
    WaitingForEntity(EntityPath),
    WaitingAssemble,
    Finished,
}

pub trait FieldResolver {}
