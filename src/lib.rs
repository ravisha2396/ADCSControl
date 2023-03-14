pub mod comms;
use std::io::{Error, ErrorKind};
use comms::Comms;

pub const ADCS_COMM_STATUS_ID:u8=0x90;
pub const ADCS_ADDRESS:u16 = 0x57;
pub const ADCS_TEMPERATURE_TLM_ID: u8 = 0xAE;

pub struct ADCSControl{
    addr: u16,
    tenbit: bool,
}

impl ADCSControl{

    // constructor to create ADCSControl object to use ADCS telemetry/telecommand methods
    pub fn new()->Result<ADCSControl, std::io::Error>{
        Ok(ADCSControl{
            addr: ADCS_ADDRESS,
            tenbit: false,
        })
    }
    // method to send a telemetry request to the ADCS from the OBC, includes error checking
    pub fn send_request(&self, tlm_id: u8, frame_length:usize)->Result<Vec<u8>, std::io::Error>{
        // create i2c error for propagation
        let status_error = Error::new(ErrorKind::Other, "i2c incomplete error, might have to retry");
        let err = Error::new(ErrorKind::Other, "error in match in send_request method, might have to retry");

        let mut conn = Comms::new(self.addr, self.tenbit);
        conn.comms_init();
        let mut buffer = vec![0;frame_length];
        // send the request and read replies
        let _result = match conn.comms_block_rx(tlm_id, buffer.as_mut_slice()){
            Ok(s) => s,
            _ => return Err(err)
        };

        // check for any telemetry read errors by sending a tlm request COMM_STATUS_ID, 
        let mut status_buf= [0u8;6];
        let _status = conn.comms_block_rx(ADCS_COMM_STATUS_ID, status_buf.as_mut_slice());
        let i2c_tlm_error_flag = match status_buf.get(3) // byte 3 has the status flags of i2c transactions
        {
            Some(s) => s,
            _ => return Err(err)
        };
        let i2c_tlm_error_flag = (i2c_tlm_error_flag>>3)&0x1;
        
        match i2c_tlm_error_flag{
            0 => Ok(buffer),
            _ => Err(status_error)
        }
    }
    // method to send a telecommand request to the ADCS from the OBC, includes error checking
    pub fn send_command(&self, command_id: u8, data: &[u8]) -> Result<(), std::io::Error> {
        // create i2c error for propagation
        
        let mut conn = Comms::new(self.addr, self.tenbit);
        conn.comms_init();

        conn.comms_block_tx(command_id, data)
    }
    // method to get adcs temperature
    pub fn request_temp(&self) -> Result<i16, std::io::Error> {

        let temp_val_err = Error::new(ErrorKind::Other, "value in returned temperature vector is incorrect");
        let command_id: u8 = ADCS_TEMPERATURE_TLM_ID;
        let result = self.send_request(command_id, 6);  // frame length of temperature TLM frame is 6 bytes (ref: CubeADCS ref manual)

        let temperature = match result{
            Ok(s) => s,
            Err(e) => return Err(e),
         };

         let temperature_slice = &temperature[..2];

         let t_low = match temperature_slice.get(0){
            Some(t) => *t,
            None => return Err(temp_val_err)
         } as u16;

         let t_high = match temperature_slice.get(1){
            Some(t) => *t,
            None => return Err(temp_val_err)
         } as u16;

         Ok(((t_high<<8)|(t_low))as i16) // upon receiving a negative i16 value, remove the sign bit and read the temperature value in the calling function
                                        // ex: res = res&0x7fff to remove the sign bit and read the negative value. res should be mutable.
    }

    // keep adding in more adcs functions as per requirements down here ..........

}