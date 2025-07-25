#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new();
    };
}
