use std::{
    collections::{HashSet, VecDeque},
    fmt::write,
    future::IntoFuture,
    sync::{Arc, Condvar, Mutex, RwLock},
    time::{Duration, Instant},
};

use ddp_rs::connection::DDPConnection;
use serde::{Deserialize, Serialize};

use super::{
    modifiers::{ModifierChainable, ModifierParameters, ModifierResult, PixelModifier},
    pixelstrip::PixelStrip,
    pixelstripcommand::PixelStripCommand,
};

struct StripAndChain {
    pixel_strip: PixelStrip,
    modifier_chain: Vec<PixelModifier>,
    to_remove: HashSet<usize>,
    connection: DDPConnection,
    display_interval: Duration,
    last_send_time: Instant,
}

impl StripAndChain {
    fn run_modifier_chain(&mut self) -> bool {
        let target_send_time = self.last_send_time.checked_add(self.display_interval);
        let now = Instant::now();

        let chosen_send_time = match target_send_time {
            Some(target) => {
                if target > now {
                    target
                } else {
                    now
                }
            }
            None => now,
        };

        let params = ModifierParameters {
            time: chosen_send_time,
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

        let now = Instant::now();
        if now < chosen_send_time {
            let duration = chosen_send_time.duration_since(now);
            //println!("Sleeping for {:?}", duration);
            std::thread::sleep(duration);
        }

        match self.pixel_strip.flush_and_write(&mut self.connection) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Couldn't write to DDP connection. {:?}", e)
            }
        }

        self.last_send_time = Instant::now();

        if remove_all {
            self.modifier_chain.clear();
        }

        self.modifier_chain.len() > 0 //Return true if the modifier chain is still populated.
    }
}

struct CommandsAndConvar {
    commands: Mutex<Vec<PixelStripCommand>>,
    condvar: Condvar,
}

pub struct PixelStripManager {
    strip_and_chain: Mutex<StripAndChain>,
    commands_and_condvar: CommandsAndConvar,
}

impl PixelStripManager {
    pub fn new(
        pixel_strip: PixelStrip,
        display_frequency: f64,
        connection: DDPConnection,
    ) -> Arc<PixelStripManager> {
        let psm = Arc::new(PixelStripManager {
            strip_and_chain: Mutex::new(StripAndChain {
                pixel_strip,
                modifier_chain: Vec::new(),
                to_remove: HashSet::new(),
                connection,
                display_interval: Duration::from_secs_f64(1.0 / display_frequency),
                last_send_time: Instant::now(),
            }),
            commands_and_condvar: CommandsAndConvar {
                commands: Mutex::new(Vec::new()),
                condvar: Condvar::new(),
            },
        });

        tokio::spawn(Self::run(psm.clone()));

        psm
    }

    fn wait_for_work(&self) {
        match self.commands_and_condvar.commands.lock() {
            Ok(lock) => match self.commands_and_condvar.condvar.wait(lock) {
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

    async fn run(self: Arc<PixelStripManager>) {
        println!("Starting pixel strip manager thread.");
        loop {
            let continue_working = match self.strip_and_chain.lock() {
                Ok(mut strip_and_chain) => {
                    match self.commands_and_condvar.commands.lock() {
                        Ok(mut commands) => {
                            for command in commands.iter() {
                                match command {
                                    PixelStripCommand::RunRainbowOscillation => {
                                        strip_and_chain.modifier_chain.clear();
                                        strip_and_chain
                                            .modifier_chain
                                            .push(PixelModifier::new_rainbow_oscillation(5000));
                                    }
                                    PixelStripCommand::RunWaveout => {
                                        eprintln!("Not implemented.");
                                    }
                                    PixelStripCommand::SinglePixel(_, pixel_values) => {
                                        eprintln!("Not implemented.");
                                    }
                                }
                            }
                            commands.clear();
                        }
                        Err(e) => {
                            eprintln!("Can't lock commands. {:?}", e);
                        }
                    }
                    strip_and_chain.run_modifier_chain()
                }
                Err(e) => {
                    eprintln!("Couldn't lock strip and chain. {:?}", e);
                    false
                }
            };

            if !continue_working {
                println!("Waiting for work.");
                self.wait_for_work();
            }
        }
    }

    pub fn queue_command(&self, command: PixelStripCommand) {
        match self.commands_and_condvar.commands.lock() {
            Ok(mut lock) => {
                println!("Notifying thread to work.");
                lock.push(command);
                self.commands_and_condvar.condvar.notify_one();
            }
            Err(e) => {
                eprintln!("Couldn't get wait. {:?}", e);
            }
        }
    }
}
