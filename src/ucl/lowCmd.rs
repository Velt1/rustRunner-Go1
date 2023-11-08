use crc::{crc32, Hasher32};
use std::collections::HashMap;
use byteorder::{ByteOrder, LittleEndian};

// Assuming `MotorModeLow`, `GaitType`, `SpeedLevel` are enums defined in `enums.rs`
// and `float_to_hex`, `encrypt_crc`, `gen_crc` are functions defined in `common.rs`
// and `BmsCmd`, `MotorCmd`, `MotorCmdArray` are structs or enums defined in `complex.rs`
// You will need to define these in Rust accordingly.

pub struct LowCmd {
    head: Vec<u8>,
    level_flag: u8,
    frame_reserve: u8,
    sn: Vec<u8>,
    version: Vec<u8>,
    band_width: Vec<u8>,
    motor_cmd: MotorCmdArray,
    bms: BmsCmd,
    wireless_remote: Vec<u8>,
    reserve: Vec<u8>,
    crc: Option<u32>,
    encrypt: bool,
}

impl LowCmd {
    pub fn new() -> LowCmd {
        LowCmd {
            head: vec![0xFE, 0xEF], // Hex FEEF
            level_flag: 0xff,
            frame_reserve: 0,
            sn: vec![0; 8],
            version: vec![0; 8],
            band_width: vec![0x3a, 0xc0], // Hex 3AC0
            motor_cmd: MotorCmdArray::new(), // Placeholder for the actual implementation
            bms: BmsCmd::new(0, vec![0, 0, 0]), // Placeholder for the actual implementation
            wireless_remote: vec![0; 40],
            reserve: vec![0; 4],
            crc: None,
            encrypt: true,
        }
    }

    pub fn build_cmd(&mut self, debug: bool) -> Vec<u8> {
        let mut cmd = vec![0; 614];
        cmd[0..2].copy_from_slice(&self.head);
        cmd[2] = self.level_flag;
        cmd[3] = self.frame_reserve;
        cmd[4..12].copy_from_slice(&self.sn);
        cmd[12..20].copy_from_slice(&self.version);
        cmd[20..22].copy_from_slice(&self.band_width);
        cmd[22..562].copy_from_slice(&self.motor_cmd.get_bytes());
        cmd[562..594].copy_from_slice(&self.bms.get_bytes());
        cmd[594..606].copy_from_slice(&self.wireless_remote);

        // ... Fill in the rest of the cmd with data from motor_cmd, bms, etc.
    //     if self.encrypt:
    //     cmd[-4:] = encryptCrc(genCrc(cmd[:-6]))
    // else:
    //     cmd[-4:] = genCrc(cmd[:-6])
        // Placeholder for the actual CRC and encryption logic
        // You will need to implement the `encrypt_crc` and `gen_crc` functions in Rust.

        if debug {
            println!("Length: {}", cmd.len());
            println!("Data: {}", cmd.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
        }

        cmd
    }

    // Assuming this is meant to be a static method to parse a `LowCmd` from bytes
    pub fn low_cmd_from_bytes(data: &[u8]) -> LowCmd {
        let mut lcmd = LowCmd::new();
        lcmd.head = data[0..2].to_vec();
        lcmd.level_flag = data[2];
        lcmd.frame_reserve = data[3];
        lcmd.sn = data[4..12].to_vec();
        lcmd.version = data[12..20].to_vec();
        lcmd.band_width = data[20..22].to_vec();
        lcmd.motor_cmd = MotorCmdArray::from_bytes(&data[22..562]);
        lcmd.bms = BmsCmd::from_bytes(&data[562..566]);
        lcmd.wireless_remote = data[566..606].to_vec();
        lcmd.encrypt = None;

        // if cmd[-4:] == encryptCrc(genCrc(cmd[:-6])):
        //     lcmd.encrypt = True

        // if cmd[-4:] == genCrc(cmd[:-6]):
        //     lcmd.encrypt = False
        // ... Parse the rest of the data into the lcmd fields

        // Placeholder for the actual CRC check and setting the encrypt field
        // You will need to implement the `encrypt_crc` and `gen_crc` functions in Rust.

        lcmd
    }
}
