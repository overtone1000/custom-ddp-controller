use std::{collections::VecDeque, sync::Arc};

use ddp_rs::connection::DDPConnection;
use serde::{Deserialize, Serialize};

use super::pixelstrip::PixelStrip;

#[derive(Serialize, Deserialize, Debug)]
pub enum PixelStripCommand
{
    RainbowOscillation
}

pub struct PixelStripManager {
    pixel_strip:PixelStrip,
    connection:DDPConnection,
    commands:VecDeque<PixelStripCommand>
}

impl  PixelStripManager
{
    pub fn new(pixel_strip:PixelStrip, connection:DDPConnection)->PixelStripManager
    {
        PixelStripManager
        {
            pixel_strip,
            connection,
            commands:VecDeque::new()
        }
    }

    pub async fn queue_command(&mut self, command:PixelStripCommand)
    {
        self.commands.push_back(command);
    }
}

#[cfg(test)]
mod tests {

    use super::PixelStripCommand;

    #[test]
    fn serialization() {
        let serialized=serde_json::to_string(&PixelStripCommand::RainbowOscillation).expect("Should serialize.");
        println!("Rainbow Oscillation: {}",serialized);
        let deserialized:PixelStripCommand=serde_json::from_str(&serialized).expect("Should deserialize.");
        println!("{:?}",deserialized);
    }
}