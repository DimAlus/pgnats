// ::pgrx::pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::prelude::pg_schema]
mod tests {
    use pgrx::prelude::*;
    use crate::funcs;

    #[pg_test]
    fn test_hello_pgnats() {
        assert_eq!("Hello, pgnats!", funcs::hello_pgnats());
    }
}
