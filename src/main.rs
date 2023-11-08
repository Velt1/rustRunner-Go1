// Import necessary libraries and modules
use std::thread;
use std::time::Duration;
pub mod ucl {
    pub mod unitreeConnection;
    pub mod highCmd;
    pub mod highState;
    pub mod common;
}

// Define constants
const HIGH_WIFI_DEFAULTS: (u16, &'static str, u16, &'static str) = (
    8090, "192.168.12.1", 8082, "192.168.12.14"
);

// Define custom data structures if needed

fn main() {
    // Print the library version
    println!("Running lib version: {}", ucl::common::lib_version());

    // Create a unitreeConnection with HIGH_WIFI_DEFAULTS
    let mut conn = ucl::unitreeConnection::new(HIGH_WIFI_DEFAULTS);
    conn.start_recv();

    let mut hcmd = ucl::highCmd::new();
    let mut hstate = ucl::highState::new();

    // Send an empty command to initialize the connection
    let cmd_bytes = hcmd.build_cmd(false);
    conn.send(&cmd_bytes);

    thread::sleep(Duration::from_secs(1)); // Sleep for some time to collect packets

    let mut motion_time = 0;

    loop {
        motion_time += 1;
        thread::sleep(Duration::from_millis(2));

        let data = conn.get_data();

        for packet in data.iter() {
            if motion_time % 100 == 0 {
                // Print information from hstate
                hstate.parse_data(packet);

                println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+");
                println!("SN [{}]:\t{}", ucl::common::byte_print(&hstate.sn), ucl::common::decode_sn(&hstate.sn));
                println!("Ver [{}]:\t{}", ucl::common::byte_print(&hstate.version), ucl::common::decode_version(&hstate.version));
                println!("SOC:\t\t\t{} %", hstate.bms.soc);
                // Implement get_voltage, get_current, and other functions if needed
                // println!("Overall Voltage:\t{} mv", get_voltage(&hstate.bms.cell_vol));
                // println!("Current:\t\t{} mA", get_current(&hstate.bms.current));
                println!("Cycles:\t\t\t{}", hstate.bms.cycle);
                println!("Temps BQ:\t\t{} °C, {}°C", hstate.bms.bq_ntc[0], hstate.bms.bq_ntc[1]);
                println!("Temps MCU:\t\t{} °C, {}°C", hstate.bms.mcu_ntc[0], hstate.bms.mcu_ntc[1]);
                println!("FootForce:\t\t{:?}", hstate.foot_force);
                println!("FootForceEst:\t\t{:?}", hstate.foot_force_est);
                println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+");
            }
        }

        // Implement motion control logic here

        // Break the loop after a certain condition (e.g., motion_time > 24000)
        if motion_time > 24000 {
            break;
        }
    }
}

// Define functions and data structures as needed
// ...

