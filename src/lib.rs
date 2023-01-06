mod msgs;

// Struct for handling hidapi context.
// Only one instance can exists at a time.
pub struct QKApi;

impl QKApi {
    fn new() -> Self {
        QKApi {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
