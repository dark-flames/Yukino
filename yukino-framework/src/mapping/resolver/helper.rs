use crate::mapping::resolver::field_resolve_cell::FieldPath;

pub fn compare_path_vector(a: &Vec<FieldPath>, b: &Vec<FieldPath>) -> bool {
    a.iter().zip(b.iter()).filter(
        |(a, b)| a !=b
    ).count() == 0
}