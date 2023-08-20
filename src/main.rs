const STR: &str = r#"
["{\"arg\":\"test\"}"]
"#;

fn main() {
    let mut str_bytes = STR.to_string().into_bytes();
    let mut r: Vec<String> = simd_json::from_slice(&mut str_bytes).unwrap();
    let simd_string = r.pop().unwrap();
    let mut r: Vec<String> = serde_json::from_str(STR).unwrap();
    let serde_string = r.pop().unwrap();
    println!("simd-json decoded string: {simd_string}\nserde-json decoded string: {serde_string}",);
    assert_eq!(simd_string, serde_string);
}
