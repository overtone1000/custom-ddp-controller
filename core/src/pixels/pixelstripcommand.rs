use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PixelValues{red:u8,green:u8,blue:u8}

#[derive(Serialize, Deserialize, Debug)]
pub enum PixelStripCommand
{
    RunRainbowOscillation,
    RunWaveout,
    SinglePixel(u32,PixelValues)
}


#[cfg(test)]
mod tests {

    use super::*;

    fn check_serialization(command:&PixelStripCommand){
        println!("Serialization test:");
        let serialized=serde_json::to_string(command).expect("Should serialize.");
        println!("   {}",serialized);
        let deserialized:PixelStripCommand=serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("   {:?}",deserialized);
    }

    #[test]
    fn serialization() {
        check_serialization(&PixelStripCommand::RunRainbowOscillation);
        check_serialization(&&PixelStripCommand::RunWaveout);
        check_serialization(&&PixelStripCommand::SinglePixel(14, PixelValues{red:255,green:7,blue:42}));
    }
}