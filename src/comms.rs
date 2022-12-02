extern crate i2c_linux;
use i2c_linux::I2c;
use std::fs::File;

pub struct Comms{
    i2c: I2c<File>,
    addr: u16,
    tenbit: bool,
}

impl Comms{

    pub fn new( addr: u16, tenbit: bool) -> Self{
        Comms{
            i2c : I2c::from_path("/dev/i2c-1").expect("error"),
            addr : addr,
            tenbit: tenbit,
        }
    } 

    pub fn comms_init(&mut self){
        // initialize
        self.i2c.smbus_set_slave_address(self.addr, self.tenbit).expect("error setting address");

        println!("I2C device setup successful!");

        
    
    }

    pub fn comms_block_tx(&mut self, command: u8, data: &[u8]){
        // Test block transfers
        // self.i2c.smbus_write_block_data(command, data).expect("error in send");

        // Serial transmit command and then data in separate i2c transfers
        self.i2c.smbus_write_byte(command).expect("command not valid!");
        for i in 0..data.len(){
            self.i2c.smbus_write_byte(data[i]).expect("data invalid");
        }
    }

    pub fn comms_block_rx(&mut self, command: u8)->u8{

        // self.i2c.smbus_read_block_data(command, buf).expect("error in read");
        self.i2c.smbus_read_byte_data(command).unwrap()
    }
    
    pub fn check_transmission(&mut self)-> bool{
        // message id = 240
        // frame length 4 bytes
        // check processed flag
        // check validity status - TC Error Status
        let mut buf =[0u8; 32];
        self.i2c.smbus_read_block_data(240, &mut buf).expect("error in ack check");


        if buf[1]&1==0 {
            return false;
        }

        else{
            if(buf[2] == 0){
                return true;
            }
            else{
                println!("Command errored with error number {}", buf[2]);
                return false;
            }
        }

    }
    //pub fn comms_write_read(&mut self, )

}


// fn main(){

//     let mut conn = Comms::new(0x53, false);
//     conn.comms_init();
//     loop{

//         println!("Sending 0xab continuously on i2c!");
//         let write_buf = [0xabu8;1];
//         conn.comms_block_tx(0x52, &write_buf);
//         println!("Reading from sensor!");
//         let mut read_buf = [0u8];

//         conn.comms_block_rx(0x51, &mut read_buf);
//     }

// }
