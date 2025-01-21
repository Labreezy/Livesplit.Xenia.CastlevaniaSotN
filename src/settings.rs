
use asr::settings::Gui;
use asr::settings::gui::Title;

#[derive(Gui)]
pub struct Settings {

    ///Second Castle
    #[default=false]
    pub second_castle: bool,

    ///Clock Rush
    #[default=false]
    pub clock_rush: bool,

    ///Library to Outer Wall
    #[default=false]
    pub library_outer_wall: bool,

    ///Bosses
    _bosses: Title,

    ///Dracula (Prologue)
    #[default=false]
    pub dracula_prologue: bool,
    ///Olrox
    #[default=false]
    pub olrox: bool,
    ///Doppleganger (10)
    #[default=false]
    pub doppleganger_10: bool,
    ///Granfaloon
    #[default=false]
    pub granfaloon: bool,
    ///Minotaur and Werewolf
    #[default=false]
    pub mino_werewolf: bool,
    ///Scylla
    #[default=false]
    pub scylla: bool,
    ///Slogra and Gaibon
    #[default=false]
    pub slogra_gaibon: bool,
    ///Hippogryph
    #[default=false]
    pub hippogryph: bool,
    ///Beelzebub
    #[default=false]
    pub beelzebub: bool,
    ///Succubus
    #[default=false]
    pub succubus: bool,
    ///Karasuman
    #[default=false]
    pub karasuman: bool,
    //Trio
    #[default=false]
    pub trio: bool,
    //Death
    #[default=false]
    pub death: bool,
    //Cerberus
    #[default=false]
    pub cerberus: bool,
    //Save Richter
    #[default=false]
    pub save_richter: bool,
    //Medusa
    #[default=false]
    pub medusa: bool,
    //Creature
    #[default=false]
    pub creature: bool,
    //Lesser Demon
    #[default=false]
    pub lesser_demon: bool,
    ///Doppleganger (40)
    #[default=false]
    pub doppleganger_40: bool,
    //Akmodan
    #[default=false]
    pub akmodan: bool,
    ///Darkwing Bat
    #[default=false]
    pub darkwing_bat: bool,
    //Galamoth
    #[default=false]
    pub galamoth: bool,
    ///Final Save
    #[default=false]
    pub final_save: bool,
    //Get Holy Glasses
    #[default=false]
    pub get_holy_glasses: bool,
    ///Meet Librarian
    #[default=false]
    pub meet_librarian: bool,
    //Meet Maria
    #[default=false]
    pub meet_maria: bool,
    ///Shaft
    #[default=false]
    pub shaft: bool,
    ///Dracula
    #[default=true]
    pub dracula_end: bool,
    
    ///Relics
    _relics: Title,


    ///Soul Of Bat
    #[default=false]
    pub soul_bat: bool,
    //Fire Of Bat
    #[default=false]
    pub fire_bat: bool,
    //Echo Of Bat
    #[default=false]
    pub echo_bat: bool,
    //Force of Echo
    #[default=false]
    pub force_echo: bool,
    ///Soul Of Wolf
    #[default=false]
    pub soul_wolf: bool,
    //Power Of Wolf
    #[default=false]
    pub power_wolf: bool,
    //Skill of Wolf
    #[default=false]
    pub skill_wolf: bool,
    ///Form of Mist
    #[default=false]
    pub form_mist: bool,
    ///Power Of Mist
    #[default=false]
    pub power_mist: bool,
    //Gas Cloud
    #[default=false]
    pub gas_cloud: bool,
    ///Cube of Zoe
    #[default=false]
    pub cube_zoe: bool,
    //Spirit Orb
    #[default=false]
    pub spirit_orb: bool,
    //Gravity Boots
    #[default=false]
    pub gravity_boots: bool,
    //Leap Stone
    #[default=false]
    pub leap_stone: bool,
    //Holy Symbol
    #[default=false]
    pub holy_symbol: bool,
    //Faerie Scroll
    #[default=false]
    pub faerie_scroll: bool,
    //Jewel of Open
    #[default=false]
    pub jewel_of_open: bool,
    //Merman Statue
    #[default=false]
    pub merman_statue: bool,
    //Bat Card
    #[default=false]
    pub bat_card: bool,
    //Ghost Card
    #[default=false]
    pub ghost_card: bool,
    //Faerie Card
    #[default=false]
    pub faerie_card: bool,
    //Demon Card
    #[default=false]
    pub demon_card: bool,
    //Sword Card
    #[default=false]
    pub sword_card: bool,
    //Heart of Vlad
    #[default=false]
    pub heart_of_vlad: bool,
    //Tooth of Vlad
    #[default=false]
    pub tooth_of_vlad: bool,
    //Rib of Vlad
    #[default=false]
    pub rib_of_vlad: bool,
    //Ring of Vlad
    #[default=false]
    pub ring_of_vlad: bool,
    //Eye of Vlad
    #[default=false]
    pub eye_of_vlad: bool
    
}