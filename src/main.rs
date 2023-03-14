use std::env;

use dora_adcs::ADCSControl;


fn main(){

    let adcs_handle = ADCSControl::new().unwrap();

    let cid = env::args().nth(1).unwrap();

    let cid: u8 = cid.parse().unwrap();

    let frame_length = env::args().nth(2).unwrap();

    let frame_length: usize = frame_length.parse().unwrap();

    let itr = env::args().nth(3).unwrap();

    let itr: usize = itr.parse().unwrap();

    println!("The telemetery requested for id: {} of length {} and repetitions {}", cid, frame_length, itr);

    // set adcs runmode to 1
    let mut data = [1u8;1];
    let _cmd_status = adcs_handle.send_command(0xa, data.as_mut_slice()).unwrap();
    println!("Sent telecommand to turn on ADCS run mode");

    let mut i=0;
    
    loop{

        println!("Sending telemetry request {} on i2c!", cid);
        // let output = adcs_handle.send_request(cid, frame_length).unwrap();
        // println!("output vector: {:?}", output);
        let temperature = adcs_handle.request_temp().unwrap();
        println!("adcs temperature: {}", temperature);

        i+=1;

        if i==itr {
            break;
        }
    }
    

}