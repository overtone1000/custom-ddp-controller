use std::collections::VecDeque;

use super::pixelstrip::PixelStrip;

#[derive(Clone)]
pub enum PixelStripCommand
{
    RainbowOscillation
}

#[derive(Clone)]
pub struct PixelStripManager {
    pixel_strip:PixelStrip,
    commands:VecDeque<PixelStripCommand>
}

impl PixelStripManager
{
    pub fn new()->PixelStripManager
    {

    }
    
    pub async fn queue_command(&mut self, command:PixelStripCommand)
    {
        self.commands.push_back(command);
    }
}