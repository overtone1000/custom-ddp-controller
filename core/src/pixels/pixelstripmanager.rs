use std::{collections::VecDeque, sync::Arc};

use ddp_rs::connection::DDPConnection;

use super::pixelstrip::PixelStrip;

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