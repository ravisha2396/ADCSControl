pub mod comms;
use comms::Comms;

pub fn send_request(command_id: u8) -> u8 {

    let mut conn = Comms::new(0x57, false);
    conn.comms_init();

    let buf = conn.comms_block_rx(command_id);
    buf

}

pub fn send_command(command_id: u8, data: &[u8]) {
    
    let mut conn = Comms::new(0x57, false);
    conn.comms_init();

    conn.comms_block_tx(command_id, data)
}

pub fn request_temp() -> u8 {

    let command_id: u8 = 0x00;       // TODO: What is this value?
    let result = send_request(command_id);  // We should do some error handling for the hardware comms (i2c)

    result
}