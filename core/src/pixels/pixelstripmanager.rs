use std::{collections::{HashSet, VecDeque}, fmt::write, sync::{Arc, RwLock}, time::Instant};

use ddp_rs::connection::DDPConnection;
use serde::{Deserialize, Serialize};

use super::{modifiers::{ModifierChainable, ModifierParameters, ModifierResult, PixelModifier}, pixelstrip::PixelStrip, pixelstripcommand::PixelStripCommand};


pub struct PixelStripManager {
    pixel_strip:PixelStrip,
    connection:DDPConnection,
    commands:VecDeque<PixelStripCommand>,
    modifier_chain:Vec<PixelModifier>,
    to_remove:HashSet<usize>
}

impl  PixelStripManager
{
    pub fn new(pixel_strip:PixelStrip, connection:DDPConnection)->Arc<RwLock<PixelStripManager>>
    {
        let retval=Arc::new(RwLock::new(PixelStripManager
        {
            pixel_strip,
            connection,
            commands:VecDeque::new(),
            modifier_chain:Vec::new(),
            to_remove:HashSet::new()
        }));

        tokio::spawn(Self::run(retval.clone()));

        retval
    }

    fn run_modifier_chain(&mut self)
    {
        let params = ModifierParameters{
            time: Instant::now(),
        };

        let mut remove_all=false;

        for (index,modifier) in &mut self.modifier_chain.iter_mut().enumerate()
        {
            match modifier.run(&mut self.pixel_strip, &params)
            {
                ModifierResult::Continue => (),
                ModifierResult::RemoveThisModifier => {
                    self.to_remove.insert(index);
                },
                ModifierResult::RemoveAllModifiers => {
                    remove_all=true;
                },
            }
        }

        if remove_all
        {
            self.modifier_chain.clear();
        }
    }

    async fn run(selfarc:Arc<RwLock<PixelStripManager>>)
    {
        loop{
            match selfarc.write()
            {
                Ok(mut writeable_self) => {
                    
                    writeable_self.run_modifier_chain();                   
                },
                Err(e) => {
                    eprintln!("Couldn't get writable access. {:?}",e);
                },
            }
        }
    }

    pub async fn queue_command(&mut self, command:PixelStripCommand)
    {
        self.commands.push_back(command);
    }
}
