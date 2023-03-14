extern crate i2c_linux;
use i2c_linux::{I2c, Message, WriteFlags, ReadFlags};
use std::{fs::File, io::{Error, ErrorKind}};

pub const TELECOMMAND_STATUS_ID: u8 = 240;
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

    pub fn comms_block_tx(&mut self, command: u8, data: &[u8])->Result<(), std::io::Error>{
        // Serial transmit command and then data in separate i2c transfers
        let _res = self.i2c.i2c_write_block_data(command, data);

        let mut buf;
        // poll tele acknowledge id till it becomes 1
        loop{
            buf = [0u8; 32];
            self.i2c.i2c_read_block_data(TELECOMMAND_STATUS_ID, buf.as_mut_slice()).expect("error in ack check");

            if buf[1]&0x1 == 0x1{
                break;
            }
        }
        // check command validity by checking error byte
        match buf[2]{
            0 => Ok(()),
            1 => Err(Error::new(ErrorKind::Other, "Invalid TC")),
            2 => Err(Error::new(ErrorKind::Other, "Incorrect Length")),
            3 => Err(Error::new(ErrorKind::Other, "Incorrect Parameter")),
            4 => Err(Error::new(ErrorKind::Other, "CRC Failed")),
            _ => Err(Error::new(ErrorKind::Other, "Unknown ADCS Error"))

        }
    }

    pub fn comms_block_rx(&mut self, command: u8, buf: &mut [u8])-> Result<usize, std::io::Error>{

        let mut msgs = [
            Message::Write {
                address: self.addr,
                data: &[command],
                flags: WriteFlags::default(),
            },
            Message::Read {
                address: self.addr,
                data: buf,
                flags: ReadFlags::default(),
            },
        ];

        return self.i2c.i2c_transfer(&mut msgs)
            .map(|_| msgs[1].len());

    }

}

