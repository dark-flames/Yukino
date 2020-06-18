use std::any::TypeId;

#[allow(dead_code)]
pub trait TypeConverter {
    type TargetType: 'static;

    fn match_type(type_id: TypeId) -> bool {
        TypeId::of::<Self::TargetType>() == type_id
    }
}