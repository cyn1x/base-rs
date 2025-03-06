mod base;

pub fn base64_encode(input: &[u8]) -> String {
    let res = base::base64::encode(input);
    res
}
