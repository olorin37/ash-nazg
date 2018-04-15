fn abc() -> String {
    String::from("abc")
}

#[cfg(test)]
mod tests {
    use abc;

    #[test]
    fn checkit() {
        assert_eq!(2+2, 4)
    }

    #[test]
    fn checkabc() {
        assert_eq!("abc".to_string(), abc())
    }
}
