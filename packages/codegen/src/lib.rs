pub fn codegen() -> String {
    "codegen".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(codegen(), "codegen".to_string());
    }
}
