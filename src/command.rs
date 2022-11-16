
use crate::comms::*;

pub struct CommandPacket<'a>{
    cmd_id: u8,
    param_list: &'a [u8],
    conn : Comms
}

impl<'a> CommandPacket<'a>{

    pub fn new(cid: u8, params: &'a [u8]) -> Self{
        CommandPacket{
            cmd_id: cid,
            param_list: params,
            conn: Comms::new(0x53, false)
        }
    }

    pub fn send_cmd(&mut self){
        self.conn.comms_block_tx(self.cmd_id, self.param_list);


    }
}


