use embassy_time::Duration;
use rusb;
use rusb::UsbContext;
use rusb::Error;
use rusb::Device;


struct Pico {
    context : Context,
    vid : u16,
    pid : u16,
    bulk_interface : u8,
    bulk_receiver : u8,
    bulk_sender : u8,
}

impl Pico {
    fn get_handle(&self) -> DeviceHandle<Context> {
        self.context
            .open_device_with_vid_pid(self.vid, self.pid)
            .expect("failed to open pico with VID and PID")
    }
}


struct PicoHandle {
    pico : Pico,
    handle : DeviceHandle<Context>
}

impl PicoHandle {
    fn write_bulk(&self, information:&[u8]) {
        let timeout_time = std::time::Duration::from_millis(1);
        //let timeout_time = std::time::Duration::from_secs(1);
        self.handle.write_bulk(pico.bulk_receiver, information, timeout_time).expect("failed to write to pico");
    }
}


//const PICO_VID : u16 = 49374;
//const PICO_PID : u16 = 51966;


fn main() {

    let context = rusb::Context::new().expect("failed to get context");

    let pico = Pico {
        context:context,
        vid:49374,
        pid:51966,
        bulk_interface:0,
        bulk_receiver:1,
        bulk_sender:129,
    };

    let pico_handle = PicoHandle {
        pico:pico,
        handle:pico.get_handle()
    };

    let buf: [u8; 4096] = [1; 4096];

    pico_handle.write_bulk(&buf);


    //let pico_handle = context.open_device_with_vid_pid(PICO_VID, PICO_PID).expect("failed to open pico with VID and PID");

    //let claim_result = pico_handle.claim_interface(0);
    
    //let end = 1;
    //let time = std::time::Duration::from_millis(1);
    //let buffer: [i32; 4096] = [0; 4096];
    
    //let write_result = pico_handle.write_bulk(end, &buffer, time);

}