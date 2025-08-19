use ddp_rs::{connection, error::DDPError};

pub struct PixelStrip{
    bytes:Vec<u8>
}

const RED_OFFSET:usize=0;
const GREEN_OFFSET:usize=1;
const BLUE_OFFSET:usize=2;
const COLORS_PER_PIXEL:usize=3;

impl PixelStrip{
    pub fn create(pixel_count:usize)->PixelStrip
    {
        let byte_count=pixel_count*COLORS_PER_PIXEL;
        let mut v = Vec::with_capacity(byte_count);
        for i in 0..byte_count
        {
            v.push(0);
        }
        PixelStrip { bytes: v }
    }

    fn set_byte(&mut self, byte_index:usize, value:u8)
    {
        match self.bytes.get_mut(byte_index)
        {
            Some(byte) => {*byte=value},
            None => eprintln!("Access error."),
        }
    }

    fn set_pixel_color(&mut self, pixel_index_start:usize, offset:usize, value:Option<u8>)
    {
        match value
        {
            Some(value)=>{self.set_byte(pixel_index_start+offset, value);},
            None=>()
        }
    }

    pub fn set_pixel(&mut self, index:usize, red:Option<u8>, green:Option<u8>, blue:Option<u8>)
    {
        let pixel_base=index*COLORS_PER_PIXEL;
        self.set_pixel_color(pixel_base,RED_OFFSET,red);        
        self.set_pixel_color(pixel_base,GREEN_OFFSET,green);
        self.set_pixel_color(pixel_base,BLUE_OFFSET,blue);
    }

    pub fn write_to_connection(&self, conn: &mut connection::DDPConnection) -> Result<usize, DDPError>
    {
        conn.write(&self.bytes)
    }
}