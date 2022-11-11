
struct Item {
    name : String,
    cost : u8,
    weight : u8,
    desc : String,
}

struct Armor {
    parent : Item,
    bonus : i8,
    max_dex : u8,
    penalty : i8,
    spell_failure : f32,
    speed_med : u8,
    speed_small : u8,
    shield : bool,

}

struct Skill {
    class : bool,
    total : u8,
    max : u8,
    ranks : u8,
    misc1 : u8,
    misc2 : u8,
    misc3 : u8,

}

struct Weapon {
    
    type : String,


}

struct Character{

    //Ability Scores
    strength : u8,
    dexterity: u8,
    contitution : u8,
    intelligence : u8,
    wisdom : u8,
    charisma : u8,

    initiative : u8,
    ac : u8,
    armor : Armor,
    weapon : Weapon,

    health : i16,

    //Saving Throws
    fortitude : u8,
    reflex : u8,
    will : u8,

    //Speed
    base : u8,
    modified : u8,
}
impl Character {
    
}


fn main() {
    println!("Hello, world!");
}
