#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(nonstandard_style)]
use ::rand::Rng;
//use ::windows;

enum Ammo {
    a_762,
    a_300blk,
    a_556,
    a_9mm,
    a_45acp,
    a_slug,
    a_buckshot,
    a_50ae,
    a_beanbag,
}
enum GunType {
    AssaultRifle,
    BattleRifle,
    Shotgun,
    Pistol,
    SMG,
    PDW,
    Rev,
    ol,
    ver}
struct Gun {
    name: String,
    ammo_type: Ammo,
    mag_size: i32,
    firerate: i32,
    gtype: GunType,
    is_primary: bool,
}
enum PickupType {
    quest,
    key,
    ammo,
    health,
    attachment,
    armor,
    gun,
}
struct Pickup {
    ptype: PickupType,
    value: i32,
}

fn main() {}
