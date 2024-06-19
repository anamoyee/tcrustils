pub mod tcr_file;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tcr_file::nothing();
        if result.is_err() {
            panic!("Error in tcr_file::nothing()");
        }
    }
}
