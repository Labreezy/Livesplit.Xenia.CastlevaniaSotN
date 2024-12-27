#![no_std]

mod auto_splitter;
mod settings;
pub mod state;

use crate::auto_splitter::*;
use crate::state::*;
use asr::future::retry;
use asr::FromEndian;
use asr::{future::next_tick, print_message,print_limited, Process, Address};
use asr::settings::Gui;

asr::async_main!(stable);
asr::panic_handler!();





async fn main() {
    let mut settings = settings::Settings::register();
    let mut custom_vars = auto_splitter_startup();
    asr::set_tick_rate(60.0);
    loop {
        let process = retry(|| Process::attach("xenia_canary.exe")).await;
        print_message("Process found!");
        process
            .until_closes(async {
                loop {
                
                auto_splitter_init(&settings);

                let mut game_state = GameState::default();
                
                loop {
                    settings.update();
                    
                    macro_rules! unwrap_or_next_tick_opt {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Some(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! unwrap_or_next_tick_res {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Ok(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! read_mem {
                        ( $name:ident, $addr:expr, $t:ty) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state.$name.update(Some(unwrap_or_next_tick_res!(
                                    process.read::<$t>($addr),
                                    &format_args!("Failed reading {}", stringify!($name))
                                ))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }
                    
                    macro_rules! read_mem_and_map {
                        ( $name:ident, $addr:expr, $t:ty, $mapper:expr) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state
                                    .$name
                                    .update(Some($mapper(unwrap_or_next_tick_res!(
                                        process.read::<$t>($addr),
                                        &format_args!("Failed reading {}", stringify!($name))
                                    )))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }

                    
                        read_mem!(map_x, MAP_X_ADDR, u8);
                        read_mem!(map_y, MAP_Y_ADDR, u8);
                        read_mem!(relic_vals, RELIC_BASE_ADDR, [u8; 28]);
                        read_mem!(bossrecord_vals, BOSSRECORD_BASE_ADDR, [u32; 26]);
                        read_mem_and_map!(boss_hp, BOSS_HP_ADDR, u32, |value: u32| {value.from_be()});
                        read_mem!(time_hours, TIME_HOURS_ADDR, u8);
                        read_mem!(time_mins, TIME_MINS_ADDR, u8);
                        read_mem!(time_secs, TIME_SECS_ADDR, u8);
                        read_mem!(time_frames, TIME_FRAMES_ADDR, u8);
                        
                        let vars = GameStatePair {
                            relic_vals, bossrecord_vals, boss_hp, map_x, map_y, time_hours, time_mins, time_secs, time_frames
                        };
                        auto_splitter_loop(&vars, &mut custom_vars, &settings);
                   
                    
                    next_tick().await;
                    }
                }
            }).await;
    }
}