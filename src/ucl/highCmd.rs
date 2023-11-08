use ucl::enums::{MotorModeHigh, GaitType, SpeedLevel};
use ucl::common::{float_to_hex, encrypt_crc, gen_crc, byte_print};
use ucl::complex::{Led, BmsCmd};

#[derive(Debug, Clone)]
pub struct HighCmd {
    head: [u8; 2],
    level_flag: u8,
    frame_reserve: u8,
    sn: [u8; 8],
    version: [u8; 8],
    band_width: [u8; 2],
    mode: MotorModeHigh,
    gait_type: GaitType,
    speed_level: SpeedLevel,
    foot_raise_height: f32,
    body_height: f32,
    position: [f32; 2],
    euler: [f32; 3],
    velocity: [f32; 2],
    yaw_speed: f32,
    bms: BmsCmd,
    led: Led,
    wireless_remote: [u8; 40],
    reserve: [u8; 4],
    crc: Option<[u8; 4]>,
    encrypt: bool,
}

impl HighCmd {
    pub fn new() -> Self {
        Self {
            head: [0xFE, 0xEF], // bytes.fromhex('FEEF')
            level_flag: 0x00,
            frame_reserve: 0,
            sn: [0; 8],
            version: [0; 8],
            band_width: [0; 2],
            mode: MotorModeHigh::Idle,
            gait_type: GaitType::Idle,
            speed_level: SpeedLevel::LowSpeed,
            foot_raise_height: 0.0,
            body_height: 0.0,
            position: [0.0, 0.0],
            euler: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0],
            yaw_speed: 0.0,
            bms: BmsCmd::new(0, [0, 0, 0]),
            led: Led::new(0, 0, 0),
            wireless_remote: [0; 40],
            reserve: [0; 4],
            crc: None,
            encrypt: false,
        }
    }

    pub fn build_cmd(&mut self, debug: bool) -> Vec<u8> {
        let mut cmd = vec![0; 129];
        cmd[0..2].copy_from_slice(&self.head);
        cmd[2] = self.level_flag;
        cmd[3] = self.frame_reserve;
        cmd[4..12].copy_from_slice(&self.sn);
        cmd[12..20].copy_from_slice(&self.version);
        cmd[20..22].copy_from_slice(&self.band_width);
        cmd[22] = self.mode as u8; // Directly using enum value as u8
        cmd[23] = self.gait_type as u8; // Same as above
        cmd[24] = self.speed_level as u8; // Same as above
        cmd[25..29].copy_from_slice(&float_to_hex(self.foot_raise_height));
        cmd[29..33].copy_from_slice(&float_to_hex(self.body_height));
        cmd[33..37].copy_from_slice(&float_to_hex(self.position[0]));
        cmd[37..41].copy_from_slice(&float_to_hex(self.position[1]));
        cmd[41..45].copy_from_slice(&float_to_hex(self.euler[0]));
        cmd[45..49].copy_from_slice(&float_to_hex(self.euler[1]));
        cmd[49..53].copy_from_slice(&float_to_hex(self.euler[2]));
        cmd[53..57].copy_from_slice(&float_to_hex(self.velocity[0]));
        cmd[57..61].copy_from_slice(&float_to_hex(self.velocity[1]));
        cmd[61..65].copy_from_slice(&float_to_hex(self.yaw_speed));
        cmd[65..69].copy_from_slice(&self.bms.get_bytes());
        cmd[69..73].copy_from_slice(&self.led.get_bytes());
        cmd[73..113].copy_from_slice(&self.wireless_remote);
        cmd[113..117].copy_from_slice(&self.reserve);

        let crc_part = &mut cmd[125..129]; // Last four bytes for CRC
        let crc_value = if self.encrypt {
            encrypt_crc(gen_crc(&cmd[..125])) // encrypt_crc and gen_crc should return [u8; 4]
        } else {
            gen_crc(&cmd[..125]) // gen_crc should return [u8; 4]
        };
        crc_part.copy_from_slice(&crc_value);

        // Debug printing
        if debug {
            eprintln!("Send Data ({}): {:?}", cmd.len(), cmd);
        }

        cmd
    }
}