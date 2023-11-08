use std::convert::TryInto;
use super::enums::MotorModeLow;
use super::common::{float_to_hex, hex_to_float, hex_to_tau, tau_to_hex, hex_to_kp, kp_to_hex, hex_to_kd, kd_to_hex};

// Here's a basic structure for Cartesian with no methods yet
#[derive(Debug, Clone, Copy)]
struct Cartesian {
    x: f32,
    y: f32,
    z: f32,
}

// Define a struct for BMS State
#[derive(Debug, Clone)]
struct BmsState {
    version_h: u8,
    version_l: u8,
    bms_status: u8,
    soc: u8, // State of Charge 0-100%
    current: i32, // mA
    cycle: u32,
    bq_ntc: i8, // x1 degrees centigrade
    mcu_ntc: i8, // x1 degrees centigrade
    cell_vol: Vec<u16>, // cell voltage mV
}

// Define a struct for BMS Command
#[derive(Debug, Clone)]
struct BmsCmd {
    off: u8,
    reserve: [u8; 3],
}

impl BmsCmd {
    fn get_bytes(&self) -> Vec<u8> {
        // Serialize the BmsCmd into bytes
        let mut bytes = Vec::new();
        bytes.push(self.off);
        bytes.extend_from_slice(&self.reserve);
        bytes
    }

    fn from_bytes(data: &[u8]) -> Self {
        // Deserialize bytes into a BmsCmd, assuming the slice is the correct size
        let off = data[0];
        let reserve = [data[1], data[2], data[3]];
        BmsCmd { off, reserve }
    }
}

// Define a struct for LED
#[derive(Debug, Clone)]
struct Led {
    r: u8,
    g: u8,
    b: u8,
}

impl Led {
    fn get_bytes(&self) -> Vec<u8> {
        // Serialize the Led into bytes
        vec![self.r, self.g, self.b, 0] // Adding a zero byte at the end as padding
    }
}

#[derive(Debug, Clone)]
struct MotorState {
    mode: u8,
    q: f32,        // current angle (unit: radian)
    dq: f32,       // current velocity (unit: radian/second)
    ddq: f32,      // current acceleration (unit: radian/second^2)
    tau_est: f32,  // current estimated output torque (unit: N.m)
    q_raw: f32,    // raw current angle (unit: radian)
    dq_raw: f32,   // raw current velocity (unit: radian/second)
    ddq_raw: f32,  // raw current acceleration
    temperature: f32,
    reserve: Vec<u8>, // Assuming reserve is a Vec<u8> for variable length data
}

#[derive(Debug, Clone)]
struct Imu {
    quaternion: (f32, f32, f32, f32), // normalized quaternion (w, x, y, z)
    gyroscope: [f32; 3],              // angular velocity (unit: rad/s)
    accelerometer: [f32; 3],          // acceleration (unit: m/s^2)
    rpy: [f32; 3],                    // roll, pitch, yaw (unit: radians)
    temperature: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotorCmd {
    mode: u8,                // Gewünschter Arbeitsmodus
    q: f32,                  // Gewünschter Winkel (Einheit: Radian)
    dq: f32,                 // Gewünschte Geschwindigkeit (Einheit: Radian/Sekunde)
    tau: f32,                // Gewünschtes Ausgangsdrehmoment (Einheit: Nm)
    kp: f32,                 // Gewünschte Positionssteifigkeit (Einheit: Nm/rad)
    kd: f32,                 // Gewünschte Geschwindigkeitssteifigkeit (Einheit: Nm/(rad/s))
    reserve: [u32; 3],       // Reservierte Daten
}

impl MotorCmd {
    // Constructor-Ersatz in Rust
    pub fn new(mode: u8, q: f32, dq: f32, tau: f32, kp: f32, kd: f32, reserve: [u32; 3]) -> MotorCmd {
        MotorCmd { mode, q, dq, tau, kp, kd, reserve }
    }

    // Äquivalent zur getBytes-Methode in Python
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.mode);
        bytes.extend_from_slice(&float_to_hex(self.q));
        bytes.extend_from_slice(&float_to_hex(self.dq));
        bytes.extend_from_slice(&tau_to_hex(self.tau));
        bytes.extend_from_slice(&kp_to_hex(self.kp));
        bytes.extend_from_slice(&kd_to_hex(self.kd));
        for &val in &self.reserve {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes
    }

    // Äquivalent zur fromBytes-Methode in Python
    pub fn from_bytes(bytes: &[u8]) -> Result<MotorCmd, &'static str> {
        if bytes.len() != 27 {
            return Err("Incorrect byte length for MotorCmd");
        }

        Ok(MotorCmd {
            mode: bytes[0],
            q: hex_to_float(&bytes[1..5]),
            dq: hex_to_float(&bytes[5..9]),
            tau: hex_to_tau(&bytes[9..11]),
            kp: hex_to_kp(&bytes[11..13]),
            kd: hex_to_kd(&bytes[13..15]),
            reserve: [
                u32::from_le_bytes([bytes[15], bytes[16], bytes[17], bytes[18]]),
                u32::from_le_bytes([bytes[19], bytes[20], bytes[21], bytes[22]]),
                u32::from_le_bytes([bytes[23], bytes[24], bytes[25], bytes[26]]),
            ],
        })
    }
}
#[derive(Debug, Clone)]
pub struct MotorCmdArray {
    fr_0: MotorCmd,
    fr_1: MotorCmd,
    fr_2: MotorCmd,
    fl_0: MotorCmd,
    fl_1: MotorCmd,
    fl_2: MotorCmd,
    rr_0: MotorCmd,
    rr_1: MotorCmd,
    rr_2: MotorCmd,
    rl_0: MotorCmd,
    rl_1: MotorCmd,
    rl_2: MotorCmd,
    unknown1: MotorCmd,
    unknown2: MotorCmd,
    unknown3: MotorCmd,
    unknown4: MotorCmd,
    unknown5: MotorCmd,
    unknown6: MotorCmd,
    unknown7: MotorCmd,
    unknown8: MotorCmd,
}

impl MotorCmdArray {
    pub fn new() -> Self {
        let default_motor_cmd = MotorCmd::new(
            MotorModeLow::Servo as u8, 
            0.0, 
            0.0, 
            0.0, 
            0.0, 
            0.0, 
            [0, 0, 0],
        );
        Self {
            fr_0: default_motor_cmd,
            fr_1: default_motor_cmd,
            fr_2: default_motor_cmd,
            fl_0: default_motor_cmd,
            fl_1: default_motor_cmd,
            fl_2: default_motor_cmd,
            rr_0: default_motor_cmd,
            rr_1: default_motor_cmd,
            rr_2: default_motor_cmd,
            rl_0: default_motor_cmd,
            rl_1: default_motor_cmd,
            rl_2: default_motor_cmd,
            unknown1: default_motor_cmd,
            unknown2: default_motor_cmd,
            unknown3: default_motor_cmd,
            unknown4: default_motor_cmd,
            unknown5: default_motor_cmd,
            unknown6: default_motor_cmd,
            unknown7: default_motor_cmd,
            unknown8: default_motor_cmd,
        }
    }

    pub fn set_motor_cmd(&mut self, motor_index: usize, cmd: MotorCmd) {
        match motor_index {
            0 => self.fr_0 = cmd,
            1 => self.fr_1 = cmd,
            2 => self.fr_2 = cmd,
            3 => self.fl_0 = cmd,
            4 => self.fl_1 = cmd,
            5 => self.fl_2 = cmd,
            6 => self.rr_0 = cmd,
            7 => self.rr_1 = cmd,
            8 => self.rr_2 = cmd,
            9 => self.rl_0 = cmd,
            10 => self.rl_1 = cmd,
            11 => self.rl_2 = cmd,
            12 => self.unknown1 = cmd,
            13 => self.unknown2 = cmd,
            14 => self.unknown3 = cmd,
            15 => self.unknown4 = cmd,
            16 => self.unknown5 = cmd,
            17 => self.unknown6 = cmd,
            18 => self.unknown7 = cmd,
            19 => self.unknown8 = cmd,
            _ => eprintln!("Invalid motor index"),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.fr_0.get_bytes());
        bytes.extend(self.fr_1.get_bytes());
        bytes.extend(self.fr_2.get_bytes());
        bytes.extend(self.fl_0.get_bytes());
        bytes.extend(self.fl_1.get_bytes());
        bytes.extend(self.fl_2.get_bytes());
        bytes.extend(self.rr_0.get_bytes());
        bytes.extend(self.rr_1.get_bytes());
        bytes.extend(self.rr_2.get_bytes());
        bytes.extend(self.rl_0.get_bytes());
        bytes.extend(self.rl_1.get_bytes());
        bytes.extend(self.rl_2.get_bytes());
        bytes.extend(self.unknown1.get_bytes());
        bytes.extend(self.unknown2.get_bytes());
        bytes.extend(self.unknown3.get_bytes());
        bytes.extend(self.unknown4.get_bytes());
        bytes.extend(self.unknown5.get_bytes());
        bytes.extend(self.unknown6.get_bytes());
        bytes.extend(self.unknown7.get_bytes());
        bytes.extend(self.unknown8.get_bytes());
        bytes
    }

    fn get_chunk(data: &[u8], i: usize) -> &[u8] {
        &data[(i - 1) * 27..i * 27]
    }

    pub fn from_bytes(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != 27 * 20 { // Annahme, dass es 20 Motoreneinträge gibt.
            return Err("Incorrect data length for MotorCmdArray");
        }

        self.fr_0 = MotorCmd::from_bytes(Self::get_chunk(data, 1))?;
        self.fr_1 = MotorCmd::from_bytes(Self::get_chunk(data, 2))?;
        self.fr_2 = MotorCmd::from_bytes(Self::get_chunk(data, 3))?;
        self.fl_0 = MotorCmd::from_bytes(Self::get_chunk(data, 4))?;
        self.fl_1 = MotorCmd::from_bytes(Self::get_chunk(data, 5))?;
        self.fl_2 = MotorCmd::from_bytes(Self::get_chunk(data, 6))?;
        self.rr_0 = MotorCmd::from_bytes(Self::get_chunk(data, 7))?;
        self.rr_1 = MotorCmd::from_bytes(Self::get_chunk(data, 8))?;
        self.rr_2 = MotorCmd::from_bytes(Self::get_chunk(data, 9))?;
        self.rl_0 = MotorCmd::from_bytes(Self::get_chunk(data, 10))?;
        self.rl_1 = MotorCmd::from_bytes(Self::get_chunk(data, 11))?;
        self.rl_2 = MotorCmd::from_bytes(Self::get_chunk(data, 12))?;
        self.unknown1 = MotorCmd::from_bytes(Self::get_chunk(data, 13))?;
        self.unknown2 = MotorCmd::from_bytes(Self::get_chunk(data, 14))?;
        self.unknown3 = MotorCmd::from_bytes(Self::get_chunk(data, 15))?;
        self.unknown4 = MotorCmd::from_bytes(Self::get_chunk(data, 16))?;
        self.unknown5 = MotorCmd::from_bytes(Self::get_chunk(data, 17))?;
        self.unknown6 = MotorCmd::from_bytes(Self::get_chunk(data, 18))?;
        self.unknown7 = MotorCmd::from_bytes(Self::get_chunk(data, 19))?;
        self.unknown8 = MotorCmd::from_bytes(Self::get_chunk(data, 20))?;
        Ok(())
    }
}