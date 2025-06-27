use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|_| "Failed to decode hex".to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut tab = bytes.to_vec();
    tab.reverse();
    tab
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    match script {
        [0x76, 0xa9, 0x14] => ScriptType::P2PKH,
        [0x00, 0x14, ..] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance -= fee;
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xAC => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            b => Err(format!("Invalid opcode: 0x{:02x}", b)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
