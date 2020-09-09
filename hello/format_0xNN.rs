/// return the number n as string in the form 0xNN
pub fn hex_format(n: u8) -> String {
    format!("{:#04x}", n)
    // replace ... with the answer
    // you can only replace those dots,
    // no modification to the part before/after the curly braces
}

#[test]
fn should_works() {
    assert_eq!(hex_format(190), "0xbe");
    assert_eq!(hex_format(128), "0x80");
    assert_eq!(hex_format(66), "0x42");
    assert_eq!(hex_format(10), "0x0a");
    assert_eq!(hex_format(3), "0x03");
}
