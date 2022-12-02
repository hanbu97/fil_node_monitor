use rand::Rng;

// max 15
pub fn gen_number_verification_code(length: usize) -> String {
    assert!(length < 15);

    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen::<u32>())[0..length].to_string()
}

#[test]
fn test_gen_verification_code() {
    for _ in 0..10 {
        let code = gen_number_verification_code(6);
        println!("{}", code);
    }
}
