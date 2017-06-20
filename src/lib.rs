#[macro_export]
macro_rules! trait_group {
    ($(trait $group:ident: $first_trait:path $(, $rest:path)+;)+) => {
        $(trait $group: $first_trait $(+ $rest)+ {}
        impl<T> $group for T where T: $first_trait $(+ $rest)+ {})+
    }
}

trait_group! {
    trait SyncSend:  Sync, Send;
    trait ReadWrite: std::io::Read, std::io::Write;
}

// Output:
// ```
// trait SyncSend: Sync + Send {}
// impl<T> SyncSend for T where T: Sync + Send {}
//
// trait ReadWrite: std::io::Read + std::io::Write {}
// impl<T> ReadWrite for T where T: std::io::Read + std::io::Write {}
// ```

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
