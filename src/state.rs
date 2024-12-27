use asr::{
    watcher::{Pair, Watcher},
    Address,
};

macro_rules! define_address {
    ($name:ident, $addr:expr) => {
        pub const $name: Address = Address::new($addr);
    };
}

define_address!(RELIC_BASE_ADDR,0x183174940);
define_address!(BOSSRECORD_BASE_ADDR,0x183173C30);
define_address!(BOSS_HP_ADDR,0x183152B00);
define_address!(MAP_X_ADDR,0x19316AFAF);
define_address!(MAP_Y_ADDR,0x18316AFB3);
define_address!(TIME_HOURS_ADDR,0x182E7F507);
define_address!(TIME_MINS_ADDR,0x182E7F505);
define_address!(TIME_SECS_ADDR,0x182E7F503);
define_address!(TIME_FRAMES_ADDR,0x182E7F501);

#[derive(Default)]
pub struct GameState {
    pub relic_vals: Watcher<[u8; 28]>,
    pub bossrecord_vals: Watcher<[u32; 26]>,
    pub boss_hp: Watcher<u32>,
    pub map_x: Watcher<u8>,
    pub map_y: Watcher<u8>,
    pub time_hours: Watcher<u8>,
    pub time_mins: Watcher<u8>,
    pub time_secs: Watcher<u8>,
    pub time_frames: Watcher<u8>,
}

#[derive()]
pub struct GameStatePair {
    pub relic_vals: Pair<[u8; 28]>,
    pub bossrecord_vals: Pair<[u32; 26]>,
    pub boss_hp: Pair<u32>,
    pub map_x: Pair<u8>,
    pub map_y: Pair<u8>,
    pub time_hours: Pair<u8>,
    pub time_mins: Pair<u8>,
    pub time_secs: Pair<u8>,
    pub time_frames: Pair<u8>,
}