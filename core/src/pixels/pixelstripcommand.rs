use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelValues {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PixelStripCommand {
    RunRandomPreview,
    RunRandomFadeout,
    RunRandomPost,
    SinglePixel(u32, PixelValues),
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_serialization(command: &PixelStripCommand) {
        println!("Serialization test:");
        let serialized = serde_json::to_string(command).expect("Should serialize.");
        println!("   {}", serialized);
        let deserialized: PixelStripCommand =
            serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("   {:?}", deserialized);
    }

    #[test]
    fn serialization() {
        check_serialization(&PixelStripCommand::RunRandomPreview);
        check_serialization(&&PixelStripCommand::RunRandomFadeout);
        check_serialization(&&PixelStripCommand::RunRandomPost);
        check_serialization(&&PixelStripCommand::SinglePixel(
            14,
            PixelValues {
                red: 255,
                green: 7,
                blue: 42,
            },
        ));
    }
}
