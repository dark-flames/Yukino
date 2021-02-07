use crate::Transaction;

#[allow(dead_code)]
pub struct QueryBuilderFactory<'t> {
    transaction: &'t Transaction
}

impl<'t> QueryBuilderFactory<'t> {
    pub fn create(transaction: &'t Transaction) -> QueryBuilderFactory<'t> {
        QueryBuilderFactory {
            transaction
        }
    }
}