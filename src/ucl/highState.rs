use super::enums::{MotorModeHigh, GaitType, SpeedLevel};
use super::common::{float_to_hex, hex_to_float, encrypt_crc, gen_crc, byte_print};
use super::complex::{Cartesian, Led, BmsState, Imu, MotorState};

#[derive(Debug, Clone)]
pub struct HighState {
    head: [u8; 2],
    level_flag: u8,
    frame_reserve: u8,
    sn: [u8; 8],
    version: [u8; 8],
    band_width: [u8; 4],
    imu: Imu,
    motor_state: Vec<MotorState>,
    bms: BmsState,
    foot_force: Vec<u16>,
    foot_force_est: Vec<u16>,
    mode: MotorModeHigh,
    progress: f32,
    gait_type: GaitType,
    foot_raise_height: f32,
    position: [f32; 2],
    body_height: f32,
    velocity: [f32; 3],
    yaw_speed: f32,
    range_obstacle: [f32; 4],
    foot_position_to_body: Vec<Cartesian>,
    foot_speed_to_body: Vec<Cartesian>,
    wireless_remote: [u8; 40],
    reserve: [u8; 4],
    crc: [u8; 4],
}

impl HighState {
    pub fn new() -> Self {
        HighState {
            head: [0; 2],
            level_flag: 0,
            frame_reserve: 0,
            sn: [0; 8],
            version: [0; 8],
            band_width: [0; 4],
            imu: Imu::default(),
            motor_state: vec![MotorState::default(); 20],
            bms: BmsState::default(),
            foot_force: vec![0; 4],
            foot_force_est: vec![0; 4],
            mode: MotorModeHigh::Idle,
            progress: 0.0,
            gait_type: GaitType::Idle,
            foot_raise_height: 0.0,
            position: [0.0, 0.0],
            body_height: 0.0,
            velocity: [0.0, 0.0, 0.0],
            yaw_speed: 0.0,
            range_obstacle: [0.0; 4],
            foot_position_to_body: vec![Cartesian::default(); 4],
            foot_speed_to_body: vec![Cartesian::default(); 4],
            wireless_remote: [0; 40],
            reserve: [0; 4],
            crc: [0; 4],
        }
    }
        // Convert data slice to BmsState
        pub fn data_to_bms_state(&self, data: &[u8]) -> BmsState {
            let version_h = data[0];
            let version_l = data[1];
            let bms_status = data[2];
            let soc = data[3];
            let current = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
            let cycle = u16::from_le_bytes([data[8], data[9]]);
            let bq_ntc = [data[10], data[11]];
            let mcu_ntc = [data[12], data[13]];
            let mut cell_vol = vec![];
            for i in (14..34).step_by(2) {
                cell_vol.push(u16::from_le_bytes([data[i], data[i + 1]]));
            }
            BmsState::new(version_h, version_l, bms_status, soc, current, cycle, bq_ntc, mcu_ntc, cell_vol)
        }
    
        // Convert data slice to Imu
        pub fn data_to_imu(&self, data: &[u8]) -> Imu {
            let quaternion = [
                hex_to_float(&data[0..4]),
                hex_to_float(&data[4..8]),
                hex_to_float(&data[8..12]),
                hex_to_float(&data[12..16]),
            ];
            let gyroscope = [
                hex_to_float(&data[16..20]),
                hex_to_float(&data[20..24]),
                hex_to_float(&data[24..28]),
            ];
            let accelerometer = [
                hex_to_float(&data[28..32]),
                hex_to_float(&data[32..36]),
                hex_to_float(&data[36..40]),
            ];
            let rpy = [
                hex_to_float(&data[40..44]),
                hex_to_float(&data[44..48]),
                hex_to_float(&data[48..52]),
            ];
            let temperature = data[52] as f32; // Assuming temperature is just a byte to f32
            Imu::new(quaternion, gyroscope, accelerometer, rpy, temperature)
        }
    
        // Convert data slice to MotorState
        pub fn data_to_motor_state(&self, data: &[u8]) -> MotorState {
            let mode = MotorModeHigh::from(data[0]); // Needs a conversion method from u8 to enum
            let q = hex_to_float(&data[1..5]);
            let dq = hex_to_float(&data[5..9]);
            let ddq = hex_to_float(&data[9..13]);
            let tau_est = hex_to_float(&data[13..17]);
            let q_raw = hex_to_float(&data[17..21]);
            let dq_raw = hex_to_float(&data[21..25]);
            let ddq_raw = hex_to_float(&data[25..29]);
            let temperature = data[29] as f32; // Assuming temperature is just a byte to f32
            let reserve = [data[30], data[31]];
            MotorState::new(mode, q, dq, ddq, tau_est, q_raw, dq_raw, ddq_raw, temperature, reserve)
        }
    
        // Parse a byte array to fill the HighState struct's fields
        pub fn parse_data(&mut self, data: &[u8]) {
            self.head = [data[0], data[1]];
            self.level_flag = data[2];
            self.frame_reserve = data[3];
            self.sn.copy_from_slice(&data[4..12]);
            self.version.copy_from_slice(&data[12..20]);
            self.band_width.copy_from_slice(&data[20..24]);
            self.imu = self.data_to_imu(&data[22..75]);
            self.motor_state.clear();
            for i in 0..20 {
                self.motor_state.push(self.data_to_motor_state(&data[75 + i * 32..107 + i * 32]));
            }
            self.bms = self.data_to_bms_state(&data[835..869]);

            self.foot_force = [
                u16::from_le_bytes([data[869], data[870]]),
                u16::from_le_bytes([data[871], data[872]]),
                u16::from_le_bytes([data[873], data[874]]),
                u16::from_le_bytes([data[875], data[876]]),
            ];

            self.foot_force_est = [
                u16::from_le_bytes([data[877], data[878]]),
                u16::from_le_bytes([data[879], data[880]]),
                u16::from_le_bytes([data[881], data[882]]),
                u16::from_le_bytes([data[883], data[884]]),
            ];

            self.mode = MotorModeHigh::from(data[885]); // Requires appropriate conversion from u8 to MotorModeHigh
            self.progress = hex_to_float(&data[886..890]);
            self.gait_type = GaitType::from(data[890]); // Requires appropriate conversion from u8 to GaitType
            self.foot_raise_height = hex_to_float(&data[891..895]);
            self.position = [
                hex_to_float(&data[895..899]),
                hex_to_float(&data[899..903]),
            ];
            self.body_height = hex_to_float(&data[907..911]);
            self.velocity = [
                hex_to_float(&data[911..915]),
                hex_to_float(&data[915..919]),
            ];
            self.yaw_speed = hex_to_float(&data[923..927]);
            
            self.range_obstacle = [
                hex_to_float(&data[927..931]),
                hex_to_float(&data[931..935]),
                hex_to_float(&data[935..939]),
                hex_to_float(&data[939..943]),
            ];

            self.foot_position_2_body = (0..4).map(|i| {
                Cartesian::new(
                    hex_to_float(&data[(i * 12) + 943..(i * 12) + 947]),
                    hex_to_float(&data[(i * 12) + 947..(i * 12) + 951]),
                    hex_to_float(&data[(i * 12) + 951..(i * 12) + 955]),
                )
            }).collect();

            self.foot_speed_2_body = (0..4).map(|i| {
                Cartesian::new(
                    hex_to_float(&data[(i * 12) + 991..(i * 12) + 995]),
                    hex_to_float(&data[(i * 12) + 995..(i * 12) + 999]),
                    hex_to_float(&data[(i * 12) + 999..(i * 12) + 1003]),
                )
            }).collect();

            self.wireless_remote.copy_from_slice(&data[1039..1079]);
            self.reserve.copy_from_slice(&data[1079..1083]);
            self.crc = Some([data[1083], data[1084], data[1085], data[1086]]);
            }
    }