
use std::mem::transmute;
use std::convert::TryInto;
use serde::Serialize;
use serde_json::json;


pub fn lib_version() -> &'static str {
    "0.2"
}

pub fn decode_sn(data: &[u8; 6]) -> (String, String) {
    let type_name = match data[0] {
        1 => "Laikago",
        2 => "Aliengo",
        3 => "A1",
        4 => "Go1",
        5 => "B1",
        _ => "UNKNOWN",
    };

    let model_name = match data[1] {
        1 => "AIR",
        2 => "PRO",
        3 => "EDU",
        4 => "PC",
        5 => "XX",
        _ => "UNKNOWN",
    };

    let product_name = format!("{}_{}", type_name, model_name);
    let id = format!("{}-{}-{}[{}]", data[2], data[3], data[4], data[5]);

    (product_name, id)
}


pub fn decode_version(data: &[u8; 6]) -> (String, String) {
    let hardware_version = format!("{}.{}.{}", data[0], data[1], data[2]);
    let software_version = format!("{}.{}.{}", data[3], data[4], data[5]);
    (hardware_version, software_version)
}

pub fn get_voltage(cell_voltages: &[f32]) -> f32 {
    cell_voltages.iter().sum()
}

pub fn float_to_hex(f: f32) -> [u8; 4] {
    unsafe { transmute(f.to_le()) }
}

pub fn hex_to_float(hex_bytes: &[u8]) -> f32 {
    let bytes: [u8; 4] = hex_bytes.try_into().expect("slice with incorrect length");
    let int_representation = u32::from_le_bytes(bytes);
    let float_representation: f32 = unsafe { transmute(int_representation.to_be()) };
    float_representation
}

pub fn fraction_to_hex(fraction: f32, neg: bool) -> [u8; 1] {
    let mut hex_value = (fraction * 256.0) as i32;
    if neg {
        hex_value = 255 + hex_value + 1;
    }
    [(hex_value & 0xFF) as u8] // Mask with 0xFF to ensure it's within byte range
}

pub fn hex_to_fraction(hex_byte: u8, neg: bool) -> f32 {
    let fraction = hex_byte as f32 / 256.0;
    if neg {
        -1.0 + fraction.round()
    } else {
        fraction.round()
    }
}

pub fn hex_to_tau(hex_bytes: &[u8]) -> f32 {
    let (int_bytes, fraction_byte) = hex_bytes.split_at(1);
    let mut int_val = i8::from_le_bytes(int_bytes.try_into().unwrap()) as i32;

    let neg = if int_val > 126 {
        int_val -= 256;
        true
    } else {
        false
    };

    int_val as f32 + hex_to_fraction(fraction_byte[0], neg)
}

pub fn kp_to_hex(kp: f32) -> [u8; 2] {
    let base = kp as i32;
    let frac = ((kp - base as f32) * 10.0).round() as i32;

    let mut val = if frac < 5 {
        (base * 32) + frac * 3
    } else {
        (base * 32) + ((frac - 1) * 3) + 4
    };

    let val_bytes = (val as u16).to_le_bytes();
    [val_bytes[1], val_bytes[0]] // Reverse the bytes to match the Python 'reverse' call
}

fn hex_to_kp(byte_arr: &[u8; 2]) -> f32 {
    let val = ((byte_arr[0] as u16) << 8) | byte_arr[1] as u16; // Reconstruct the value (assumes big-endian input)
    let base = val / 32;
    let remainder = val % 32;
    let frac = if remainder < 15 {
        remainder as f32 / 3.0
    } else {
        (remainder as f32 - 4.0) / 3.0 + 1.0
    };

    base as f32 + (frac / 10.0) // Combine the base and the fractional part
}

fn get_hex_frac(frac: f32) -> char {
    match frac {
        0.0 => '0',
        0.1 => '1',
        0.2 => '3',
        0.3 => '4',
        0.4 => '6',
        0.5 => '8',
        0.6 => '9',
        0.7 => 'b',
        0.8 => 'c',
        0.9 => 'e',
        _ => '0',
    }
}
fn kd_to_hex(decimal: f32) -> [u8; 2] {
    let integer_part = decimal as u16;
    let fractional_part = ((decimal.fract() * 10.0).round() as u16) % 10; // Get only one digit after decimal
    let hex_fractional_part = get_hex_frac(fractional_part as f32 / 10.0); // Convert back to fraction

    // Format and parse the integer part, combine with fractional hex representation
    let mut kd = format!("{:03x}{}", integer_part, hex_fractional_part).into_bytes();
    kd.reverse();
    let kd_array: [u8; 2] = [kd[2], kd[3]]; // Get last two bytes after reverse
    kd_array
}

fn get_hex_frac(fraction: f32) -> char {
    match fraction {
        0.0 => '0',
        0.1 => '1',
        0.2 => '3',
        0.3 => '4',
        0.4 => '6',
        0.5 => '8',
        0.6 => '9',
        0.7 => 'b',
        0.8 => 'c',
        0.9 => 'e',
        _ => '0',
    }
}

fn get_frac_hex(frac: char) -> f32 {
    match frac {
        '0' => 0.0,
        '1' => 0.1,
        '3' => 0.2,
        '4' => 0.3,
        '6' => 0.4,
        '8' => 0.5,
        '9' => 0.6,
        'b' => 0.7,
        'c' => 0.8,
        'e' => 0.9,
        _ => 0.0,
    }
}

fn hex_to_kd(hex_bytes: &[u8; 2]) -> f32 {
    let hex_string = format!("{:02x}{:02x}", hex_bytes[0], hex_bytes[1]); // Convert to hex string
    let int_part = u16::from_str_radix(&hex_string[0..3], 16).expect("Parse int error");
    let frac_part = get_frac_hex(hex_string.chars().nth(3).unwrap());

    int_part as f32 + frac_part
}
pub fn gen_crc(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for chunk in bytes.chunks(4) {
        let j = u32::from_le_bytes(chunk.try_into().unwrap());
        for b in 0..32 {
            let x = (crc >> 31) & 1;
            crc <<= 1;
            crc ^= ((x ^ ((j >> (31 - b)) & 1)) * 0x04C11DB7) as u32;
        }
    }
    crc
}

pub fn encrypt_crc(mut crc_val: u32) -> [u8; 4] {
    let xor_val = 0xEDCAB9DE;
    crc_val ^= xor_val;

    // Byte swapping
    let bytes = crc_val.to_le_bytes();
    [bytes[1], bytes[2], bytes[3], bytes[0]]
}
fn byte_print(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join("")
}

fn dump_obj<T>(obj: &T)
where
    T: Serialize,
{
    let serialized = serde_json::to_string_pretty(obj).unwrap();
    println!("{}", serialized);
}

fn pretty_print_obj<T>(obj: &T, indent: usize, border: bool)
where
    T: serde::Serialize,
{
    let obj_str = serde_json::to_string_pretty(obj).unwrap_or_else(|_| "Invalid object".to_string());
    let indent_str = " ".repeat(indent);
    let border_line = "=".repeat(90);

    if border {
        println!("{}", border_line);
    }
    println!("{}{}", indent_str, obj_str);
    if border {
        println!("{}", border_line);
    }
}