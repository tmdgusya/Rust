pub mod client;
pub mod networks;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        client::connect();
    }
}
