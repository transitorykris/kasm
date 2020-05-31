use kasm;

#[test]
fn generate_null_machine_code() {
    let is = kasm::instructions::generate_instruction_set();
    let p = kasm::pass1::Program::new();
    let mc = match kasm::pass2::pass2(is, p) {
        Ok(mc) => mc,
        Err(_) => panic!("This should never error"),
    };
    assert_eq!(0, mc.len());
}

#[test]
fn generate_simple_machine_code() {
    let is = kasm::instructions::generate_instruction_set();
    let mut p = kasm::pass1::Program::new();
    p.code.push(
        kasm::pass1::CodeTableEntry{
            address: 0x1234,
            content: kasm::pass1::Content::Code(kasm::pass1::Code{
                mnemonic: kasm::instructions::Mnemonic::LDA,
                address_mode: kasm::instructions::AddressMode::Absolute,
                value: kasm::instructions::Value::U8(0xab),
            }),
        }
    );
    let mc = match kasm::pass2::pass2(is, p) {
        Ok(mc) => mc,
        Err(_) => panic!("This should never error"),
    };
    assert_eq!(mc[0x0000], 0x00);
    assert_eq!(mc[0x1233], 0x00);
    assert_eq!(mc[0x1234], 0xad);
    assert_eq!(mc[0x1235], 0xab);
}