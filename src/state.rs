use asr::{
    watcher::{Pair, Watcher},
    Address,
};

macro_rules! define_address {
    ($name:ident, $addr:expr) => {
        pub const $name: Address = Address::new($addr);
    };
}

macro_rules! define_offset_addr {
    ($name:ident, $off:expr) => {
        pub const $name: u64 = $off - 0x82E7B284; //constant from pre-title update, changes after
    };
}

#[derive(Default, Clone, Copy)]
pub struct LocationPair {
    pub enabled: bool,
    pub has_split: bool,
    pub second_castle: bool,
    pub old_x: u8,
    pub old_y: u8,
    pub new_x: u8,
    pub new_y: u8,
}


pub static DATA_START_OFFSET: u64 = 0x82896078;
//define_address!(DATA_START_PTR, 0x182896078);
define_offset_addr!(RELIC_BASE_ADDR,0x83174940);
define_offset_addr!(BOSSRECORD_BASE_ADDR,0x83173C30);
define_offset_addr!(BOSS_HP_ADDR,0x83152B02);
define_offset_addr!(MAP_X_ADDR,0x8316AFAF);
define_offset_addr!(MAP_Y_ADDR,0x8316AFB3);
define_offset_addr!(TIME_HOURS_ADDR,0x82E7F507);
define_offset_addr!(TIME_MINS_ADDR,0x82E7F505);
define_offset_addr!(TIME_SECS_ADDR,0x82E7F503);
define_offset_addr!(TIME_FRAMES_ADDR,0x82E7F501);
define_offset_addr!(SECOND_CASTLE_ADDR,0x83162358);

#[derive(Default)]
pub struct GameState {
    pub relic_vals: Watcher<[u8; 28]>,
    pub bossrecord_vals: Watcher<[u32; 26]>,
    pub boss_hp: Watcher<i16>,
    pub map_x: Watcher<u8>,
    pub map_y: Watcher<u8>,
    pub time_hours: Watcher<u8>,
    pub time_mins: Watcher<u8>,
    pub time_secs: Watcher<u8>,
    pub time_frames: Watcher<u8>,
    pub second_castle: Watcher<u8>,
}

#[derive()]
pub struct GameStatePair {
    pub relic_vals: Pair<[u8; 28]>,
    pub bossrecord_vals: Pair<[u32; 26]>,
    pub boss_hp: Pair<i16>,
    pub map_x: Pair<u8>,
    pub map_y: Pair<u8>,
    pub time_hours: Pair<u8>,
    pub time_mins: Pair<u8>,
    pub time_secs: Pair<u8>,
    pub time_frames: Pair<u8>,
    pub second_castle: Pair<u8>,
}