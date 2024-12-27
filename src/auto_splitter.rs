use super::state::GameStatePair;
use crate::{settings::Settings};
use asr::{print_message, timer::{self, TimerState}};
pub struct CustomVars {
    relic_split_mask: [bool; 28],
    boss_split_mask: [bool; 26],
    dracula_started: bool
}

pub fn auto_splitter_startup() -> CustomVars {
    print_message("SotN Xenia Autosplitter - Loaded, waiting for xenia");

    CustomVars { 
        relic_split_mask: core::array::from_fn(|_| false), 
        boss_split_mask: core::array::from_fn(|_| false), 
        dracula_started: false 
    }
}

pub fn auto_splitter_init(_settings: &Settings) {
    print_message("SotN Xenia Autosplitter - Attached to process, beginning main auto splitter loop");
}


pub fn auto_splitter_start( 
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    settings: &Settings
) -> bool {
    custom_vars.dracula_started = false;
    
    macro_rules! set_relic_mask_if_setting {
        ( $setting:ident, $number:expr ) => {
            if settings.$setting {
                custom_vars.relic_split_mask[$number] = true
            }
        };
    }

    macro_rules! set_boss_mask_if_setting {
        ( $setting:ident, $number:expr ) => {
            if settings.$setting {
                custom_vars.boss_split_mask[$number] = true
            }
        };
    }

    set_relic_mask_if_setting!(soul_bat, 0);
    set_relic_mask_if_setting!(fire_bat, 1);
    set_relic_mask_if_setting!(echo_bat, 2);
    set_relic_mask_if_setting!(force_echo, 3);
    set_relic_mask_if_setting!(soul_wolf, 4);
    set_relic_mask_if_setting!(power_wolf, 5);
    set_relic_mask_if_setting!(skill_wolf, 6);
    set_relic_mask_if_setting!(form_mist, 7);
    set_relic_mask_if_setting!(power_mist, 8);
    set_relic_mask_if_setting!(gas_cloud, 9);
    set_relic_mask_if_setting!(cube_zoe, 10);
    set_relic_mask_if_setting!(spirit_orb, 11);
    set_relic_mask_if_setting!(gravity_boots, 12);
    set_relic_mask_if_setting!(leap_stone, 13);
    set_relic_mask_if_setting!(holy_symbol, 14);
    set_relic_mask_if_setting!(faerie_scroll, 15);
    set_relic_mask_if_setting!(jewel_of_open, 16);
    set_relic_mask_if_setting!(merman_statue, 17);
    set_relic_mask_if_setting!(bat_card, 18);
    set_relic_mask_if_setting!(ghost_card, 19);
    set_relic_mask_if_setting!(faerie_card, 20);
    set_relic_mask_if_setting!(demon_card, 21);
    set_relic_mask_if_setting!(sword_card, 22);
    set_relic_mask_if_setting!(heart_of_vlad, 23);
    set_relic_mask_if_setting!(tooth_of_vlad, 24);
    set_relic_mask_if_setting!(rib_of_vlad, 25);
    set_relic_mask_if_setting!(ring_of_vlad, 26);
    set_relic_mask_if_setting!(eye_of_vlad, 27);

    set_boss_mask_if_setting!(dracula_prologue, 0);
    set_boss_mask_if_setting!(olrox, 1);
    set_boss_mask_if_setting!(doppleganger_10, 2);
    set_boss_mask_if_setting!(granfaloon, 3);
    set_boss_mask_if_setting!(mino_werewolf, 4);
    set_boss_mask_if_setting!(scylla, 5);
    set_boss_mask_if_setting!(slogra_gaibon, 6);
    set_boss_mask_if_setting!(hippogryph, 7);
    set_boss_mask_if_setting!(beelzebub, 8);
    set_boss_mask_if_setting!(succubus, 9);
    set_boss_mask_if_setting!(karasuman, 10);
    set_boss_mask_if_setting!(trio, 11);
    set_boss_mask_if_setting!(death, 12);
    set_boss_mask_if_setting!(cerberus, 13);
    set_boss_mask_if_setting!(save_richter, 14);
    set_boss_mask_if_setting!(medusa, 15);
    set_boss_mask_if_setting!(creature, 16);
    set_boss_mask_if_setting!(lesser_demon, 17);
    set_boss_mask_if_setting!(doppleganger_40, 18);
    set_boss_mask_if_setting!(akmodan, 19);
    set_boss_mask_if_setting!(darkwing_bat, 20);
    set_boss_mask_if_setting!(galamoth, 21);
    set_boss_mask_if_setting!(final_save, 22);
    set_boss_mask_if_setting!(get_holy_glasses, 23);
    set_boss_mask_if_setting!(meet_librarian, 24);
    set_boss_mask_if_setting!(meet_maria, 25);
    


    let richter_control_start = vars.time_hours.current == 0 && vars.time_mins.current == 0 && vars.time_secs.current == 0 && vars.time_frames.current > 10;

    return richter_control_start;
}

pub fn auto_splitter_split(
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    settings: &Settings
) -> bool {
    macro_rules! split_if_true {
        ( $e:expr ) => {
            if $e {
                return true;
            }
        };
    }

    if settings.dracula_end {
        split_if_true!(custom_vars.dracula_started && vars.boss_hp.old < 9999 && vars.boss_hp.current < 1 && vars.map_x.current == 31 && vars.map_y.current == 30);
    }

    for i in 0..custom_vars.relic_split_mask.len(){
        if custom_vars.relic_split_mask[i]
            && (vars.relic_vals.current[i] == 3 && vars.relic_vals.old[i] != 3)
            {
                print_message("Relic split");
                return true;
            }
    }

    for i in 0..custom_vars.boss_split_mask.len(){
        if custom_vars.boss_split_mask[i]
            && (vars.bossrecord_vals.current[i] > 0 && vars.bossrecord_vals.old[i] == 0)
            {
                print_message("Boss Record Split");
                return true;
            }
    }
    return false;
}

pub fn auto_splitter_update(
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    _settings: &Settings
) {
    if vars.boss_hp.current == 10000 && vars.map_x.current == 31 && vars.map_x.current == 30 {
        if !custom_vars.dracula_started {
            print_message("Dracula Fight Started");
            custom_vars.dracula_started = true;
        }
    }
}

// Adapted from the SCU autosplitter by Jujstme
pub fn auto_splitter_loop(vars: &GameStatePair, custom_vars: &mut CustomVars, settings: &Settings) {
    // Splitting logic. Adapted from OG LiveSplit:
    // Order of execution
    // 1. update() will always be run first. There are no conditions on the execution of this action.
    auto_splitter_update(&vars, custom_vars, &settings);

    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        
        if auto_splitter_split(&vars, custom_vars, &settings) {
            timer::split();
        }
    }
    

    // 4. If the timer is currently not running (and not paused), then the start action will be run.
    if timer::state() == TimerState::NotRunning {
        if auto_splitter_start(&vars, custom_vars, &settings) {
            timer::start();
        }
    }
}