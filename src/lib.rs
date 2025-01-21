#![no_std]

mod auto_splitter;
mod settings;
pub mod state;



use crate::auto_splitter::*;
use crate::state::*;

use asr::future::retry;

use asr::print_message;
use asr::FromEndian;
use asr::{future::next_tick, print_limited, Process, Address};
use asr::settings::Gui;

asr::async_main!(stable);
asr::panic_handler!();



fn find_xenia_start(process: &Process) -> Option<Address> {
    let ram_base: Address;
    if !process.is_open() {
        return None;
    }
    for i in 32..40 {
        let tempaddr: u64 = 1 << i;
        let base_module = Address::new(tempaddr + 0x82000000);
        print_limited::<128>(&format_args!("Base module {:X}", base_module.value()));
            let val = process.read::<u16>(base_module);  {
                let Ok(ok_val) = val else {
                    print_message("val fail");
                    continue;
                };
                if ok_val == 0x5A4D {
                    let elf_offset = process.read::<u32>(base_module+0x3C);
                    let Ok(elf_offset_ok) = elf_offset else {
                        print_message("elf offset fail");
                        continue;
                    };
                    print_limited::<128>(&format_args!("PE offset {:X}", elf_offset_ok));
                    let pe = process.read::<u16>(base_module+elf_offset_ok);
                    let Ok(pe_ok) = pe else {
                        print_message("pe read fail");
                        
                        continue;
                    };
                     //0x4550 = PE
                    if pe_ok == 0x4550 {
                        ram_base = Address::new(tempaddr);
                        print_limited::<128>(&format_args!("RAM Base found at 0x{:X}", ram_base.value()));
                        return Some(ram_base)
                    }
                }   
                
            }
    }
    None
}

async fn main() {
    let mut settings = settings::Settings::register();
    let mut custom_vars = auto_splitter_startup();
    asr::set_tick_rate(30.0);
    loop {
        let process = retry(|| Process::attach("xenia_canary.exe")).await;
        let mut xenia_mem_start:Address = Address::new(0); 
        while xenia_mem_start.value() == 0 {
            match find_xenia_start(&process) {
                Some(x) => { xenia_mem_start = x;
                    print_limited::<128>(&format_args!("{:X}",xenia_mem_start.value()));
                    break;
                },
                _ => continue
            }
        }
        let future = async {
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
                                    &format_args!("Failed reading {} at {:X}", stringify!($name), $addr.value())
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
                                        &format_args!("Failed reading {} at {:X}", stringify!($name), $addr.value())
                                    )))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }
                    let base_ptr: u32 = unwrap_or_next_tick_res!(
                        process.read::<u32>(xenia_mem_start.add(DATA_START_OFFSET)),
                    &format_args!("Failed reading base pointer"));
                    
                    let data_start_addr: u64  = xenia_mem_start.value() + base_ptr.from_be() as u64;
                    let data_start: Address = Address::new(data_start_addr);
                    
                    macro_rules! read_offset_mem {
                        ($name:ident, $off:expr, $t:ty) => {
                            read_mem!($name, data_start.add($off), $t);
                        };
                    }
                    macro_rules! read_offset_mem_and_map {
                        ( $name:ident, $off:expr, $t:ty, $mapper:expr) => {
                            read_mem_and_map!($name, data_start.add($off), $t, $mapper);
                        };
                    }
                        read_offset_mem!(map_x, MAP_X_ADDR, u8);
                        read_offset_mem!(map_y, MAP_Y_ADDR, u8);
                        read_offset_mem!(relic_vals, RELIC_BASE_ADDR, [u8; 28]);
                        read_offset_mem!(bossrecord_vals, BOSSRECORD_BASE_ADDR, [u32; 26]);
                        read_offset_mem_and_map!(boss_hp, BOSS_HP_ADDR, i16, |value: i16| {value.from_be()});
                        read_offset_mem!(time_hours, TIME_HOURS_ADDR, u8);
                        read_offset_mem!(time_mins, TIME_MINS_ADDR, u8);
                        read_offset_mem!(time_secs, TIME_SECS_ADDR, u8);
                        read_offset_mem!(time_frames, TIME_FRAMES_ADDR, u8);
                        read_offset_mem!(second_castle, SECOND_CASTLE_ADDR, u8);
                        let vars = GameStatePair {
                            relic_vals, bossrecord_vals, boss_hp, map_x, map_y, time_hours, time_mins, time_secs, time_frames, second_castle
                        };
                        auto_splitter_loop(&vars, &mut custom_vars, &settings);
                   
                    
                    next_tick().await;
                    }
                }
            };
        process
            .until_closes(future).await;
    }
}
