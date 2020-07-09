use super::field_resolve_cell::FieldPath;

pub fn compare_path_vector(a: &[FieldPath], b: &[FieldPath]) -> bool {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count() == 0
}
