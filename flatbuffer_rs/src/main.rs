use std::fs::File;
use std::io::prelude::*;
use flatbuffers;

// Import generated code
#[allow(dead_code, unused_imports)]
#[path = "../../flatbuffer/monster_generated.rs"]
mod monster_generated;

use monster_generated::my_game::sample::*;

fn main() {

    // Build a serialized buffer (initialize with 1024 bytes)
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);


    // Serialize some weapons for the Monster: A 'sword' and an 'axe'
    let weapon_one_name = builder.create_string("Sword");
    let weapon_two_name = builder.create_string("Axe");

    // Use the 'Weapon::create' to create Weapons
    let sword = Weapon::create(&mut builder, &WeaponArgs {
        name: Some(weapon_one_name),
        damage: 3,
    });
    let axe = Weapon::create(&mut builder, &WeaponArgs {
        name: Some(weapon_two_name),
        damage: 5,
    });

    // Name of monster
    let name = builder.create_string("Orc");
    let inventory = builder.create_vector(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Create a FlatBuffer 'vector' that contains offsets to the sword and axe we created above
    let weapons = builder.create_vector(&[sword, axe]);

    // Create the path vector of Vec3 objects
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(4.0, 5.0, 6.0);
    let path = builder.create_vector(&[x, y]);

    // Create the monster using the 'Monster::create' helper function.
    // This function accepts a 'MonsterArgs' struct, which supplies all of the data needed to build a 'Monster'
    // To supply empty/default fields, just use the Rust default() function, as demonstrated below
    let orc = Monster::create(&mut builder, &MonsterArgs {
        pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
        mana: 150,
        hp: 80,
        name: Some(name),
        inventory: Some(inventory),
        color: Color::Red,
        weapons: Some(weapons),
        equipped_type: Equipment::Weapon,
        equipped: Some(axe.as_union_value()),
        path: Some(path),
        ..Default::default()
    });

    // Call 'finish()' to instruct the builder that this monster is complete
    builder.finish(orc, None);

    // This must be called after finish()
    // 'finished_data' returns a byte slice
    let buf = builder.finished_data();

    // Save binary file with flatbuffer
    let mut file = File::create("/tmp/flatbuffer.dat").unwrap();
    if let Err(e) = file.write_all(buf) { 
        println!("Error {:?} writing file", e);
    } else {
        println!("File saved");
    }


}
