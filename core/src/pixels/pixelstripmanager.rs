use std::{
    collections::{HashSet, VecDeque},
    fmt::write,
    future::IntoFuture,
    sync::{Arc, Condvar, Mutex, RwLock},
    time::Instant,
};

use ddp_rs::connection::DDPConnection;
use serde::{Deserialize, Serialize};

use super::{
    modifiers::{ModifierChainable, ModifierParameters, ModifierResult, PixelModifier},
    pixelstrip::PixelStrip,
    pixelstripcommand::PixelStripCommand,
};

pub struct MutCondvar {
    mutex: Mutex<()>,
    condvar: Condvar,
}

impl MutCondvar {
    async fn notify_thread_of_work(&mut self) {
        match self.mutex.lock() {
            Ok(_) => {
                println!("Notifying thread to work.");
                self.condvar.notify_one();
            }
            Err(e) => {
                eprintln!("Couldn't get wait. {:?}", e);
            }
        }
    }

    async fn wait_for_work(&mut self) {
        match self.mutex.lock() {
            Ok(lock) => match self.condvar.wait(lock) {
                Ok(_) => println!("Done waiting for work."),
                Err(e) => {
                    eprintln!("Mutex lock error. {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Couldn't get wait. {:?}", e);
            }
        }
    }
}

pub struct PixelStripManager {
    pixel_strip: PixelStrip,
    connection: DDPConnection,
    commands: VecDeque<PixelStripCommand>,
    modifier_chain: Vec<PixelModifier>,
    to_remove: HashSet<usize>,
}

impl PixelStripManager {
    pub fn new(
        pixel_strip: PixelStrip,
        connection: DDPConnection,
    ) -> (Arc<RwLock<PixelStripManager>>, {
        let psm = Arc::new(RwLock::new(PixelStripManager) {
            pixel_strip,
            connection,
            commands: VecDeque::new(),
            modifier_chain: Vec::new(),
            to_remove: HashSet::new(),
        }));

        let thread = tokio::spawn(Self::run(retval.clone()));
        thread.into_future();

        retval
    }

    fn run_modifier_chain(&mut self) -> bool {
        let params = ModifierParameters {
            time: Instant::now(),
        };

        let mut remove_all = false;

        for (index, modifier) in &mut self.modifier_chain.iter_mut().enumerate() {
            match modifier.run(&mut self.pixel_strip, &params) {
                ModifierResult::Continue => (),
                ModifierResult::RemoveThisModifier => {
                    self.to_remove.insert(index);
                }
                ModifierResult::RemoveAllModifiers => {
                    remove_all = true;
                }
            }
        }

        if remove_all {
            self.modifier_chain.clear();
        }

        self.modifier_chain.len() > 0 //Return true if the modifier chain is still populated.
    }

    async fn run(selfarc: Arc<RwLock<PixelStripManager>>, wait: Arc<MutCondvar>) {
        loop {
            match selfarc.write() {
                Ok(mut writeable_self) => {
                    match writeable_self.wait.mutex.lock() {
                        Ok(_) => {
                            writeable_self.wait.condvar.notify_one();
                        }
                        Err(e) => {
                            eprintln!("Couldn't get wait. {:?}", e);
                        }
                    }

                    let continue_working = writeable_self.run_modifier_chain();
                    if !continue_working {
                        writeable_self.wait_for_work();
                    }
                }
                Err(e) => {
                    eprintln!("Couldn't get writable access. {:?}", e);
                }
            };
        }
    }

    pub async fn queue_command(&mut self, command: PixelStripCommand) {
        self.commands.push_back(command);
        self.notify_thread_of_work();
    }
}
