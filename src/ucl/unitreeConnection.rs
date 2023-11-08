use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const LISTEN_PORT: u16 = 8090;
const SEND_PORT_LOW: u16 = 8007;
const SEND_PORT_HIGH: u16 = 8082;

const LOCAL_IP_WIFI: &str = "192.168.12.14";
const LOCAL_IP_ETH: &str = "192.168.123.14";
const ADDR_WIFI: &str = "192.168.12.1";
const ADDR_LOW: &str = "192.168.123.10";
const ADDR_HIGH: &str = "192.168.123.161";

const LOW_WIRED_DEFAULTS: (&str, &str, u16, &str) = (ADDR_LOW, LOCAL_IP_ETH, SEND_PORT_LOW, LOCAL_IP_ETH);
const LOW_WIFI_DEFAULTS: (&str, &str, u16, &str) = (ADDR_LOW, LOCAL_IP_WIFI, SEND_PORT_LOW, LOCAL_IP_WIFI);
const HIGH_WIRED_DEFAULTS: (&str, &str, u16, &str) = (ADDR_HIGH, LOCAL_IP_ETH, SEND_PORT_HIGH, LOCAL_IP_ETH);
const HIGH_WIFI_DEFAULTS: (&str, &str, u16, &str) = (ADDR_WIFI, LOCAL_IP_WIFI, SEND_PORT_HIGH, LOCAL_IP_WIFI);

struct UnitreeConnection {
    socket: UdpSocket,
    data: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl UnitreeConnection {
    fn new(local_ip: IpAddr, listen_port: u16, send_addr: SocketAddr) -> Self {
        let socket = UdpSocket::bind((local_ip, listen_port)).expect("Couldn't bind to address");
        socket.set_read_timeout(Some(Duration::from_secs(1))).expect("set_read_timeout call failed");
        UnitreeConnection {
            socket,
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn start_recv(&self) {
        let data = Arc::clone(&self.data);
        let socket = self.socket.try_clone().expect("Couldn't clone the socket");
        thread::spawn(move || {
            let mut buffer = [0; 2048];
            while let Ok((size, _)) = socket.recv_from(&mut buffer) {
                let mut data_lock = data.lock().unwrap();
                data_lock.push(buffer[..size].to_vec());
            }
        });
    }

    fn send(&self, send_addr: SocketAddr, cmd: &[u8]) {
        self.socket.send_to(cmd, send_addr).expect("Couldn't send data");
    }

    fn get_data(&self) -> Vec<Vec<u8>> {
        let mut data_lock = self.data.lock().unwrap();
        let ret = data_lock.clone();
        data_lock.clear();
        ret
    }
}