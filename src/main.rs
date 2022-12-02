//mod command;
use std::env;
use comms::Comms;
mod comms;


fn main(){

    let cid = env::args().nth(1).unwrap();

    let cid: u8 = cid.parse().unwrap();

    println!("The telemetery requested for id: {}", cid);

    
    let mut conn = Comms::new(0x57, false);
    conn.comms_init();
    let mut i=0;
    
    loop{

        println!("Sending telemetry request {} on i2c!", cid);
        // let write_buf = [0xabu8;1];
        // conn.comms_block_tx(0x52, &write_buf);
        // println!("Reading from sensor!");
        //let mut read_buf = [0u8;32];

        // conn.comms_block_rx(cid, &mut read_buf);
        let buf = conn.comms_block_rx(cid);
        println!("Read id value: {}", buf);
        //, {}, {}, {} read_buf[1], read_buf[2], read_buf[3]
        i+=1;

        if i==30 {
            break;
        }
    }
    

}