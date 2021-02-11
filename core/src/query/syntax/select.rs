use crate::query::expr::DatabaseIdent;

pub enum SelectItem {
    All,
    Ident(DatabaseIdent),
    Alias{ ident: DatabaseIdent, alias: String },
}