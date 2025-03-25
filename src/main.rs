use std::{collections::HashMap, env};

use rand::{self, Rng};

type LootTable = Vec<(String, f32)>;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len().lt(&1usize) {
        println!("Informa o nome do boss, disponiveis: Stalker, Deva, Kaari e Profane.");
        return;
    }

    let boss = &args[1].to_lowercase();
    let interations = &args[2].parse::<u32>().unwrap_or(1);

    let mut loot: HashMap<String, u32> = HashMap::new();

    for _ in 1..=interations.to_owned() {
        let reuslt = roll_loot(boss.to_string());
        *loot.entry(reuslt.0).or_insert(0) += reuslt.1;
    }

    println!("");
    loot.iter().for_each(|(item, quantity)| {
        println!("{}: {}x", item, quantity);
    });
}

fn roll_loot(table: String) -> (String, u32) {
    let roll = rand::rng().random_range(0.0..100.0);

    let table = match table.as_str() {
        "stalker" => create_stalker_loot_table(),
        "deva" => create_deva_loot_table(),
        "kaari" => create_kaari_loot_table(),
        "profane" => create_profane_loot_table(),
        _ => panic!("Boss não encontrado!"),
    };

    for (item, chance) in table {
        if roll < chance {
            return (item, 1);
        }
    }

    unreachable!("No valid loot determined");
}

fn create_stalker_loot_table() -> LootTable {
    vec![
        ("Small Stalker Soul Shield Chest".to_string(), 35.),
        ("Medium Stalker Soul Shield Chest".to_string(), 65.), // 35 + 30  = 65
        ("Stalker Hat".to_string(), 69.),                      // 65 + 4 = 69
        ("Stalker Charm".to_string(), 73.),                    // 69 + 4 = 73
        ("Stalker Raimen".to_string(), 77.),                   // 73 + 4 = 77
        ("Viridian Coast Stalker Jiangshi Card".to_string(), 77.05), // 77 + 0.05 = 77.05
        ("Stalker Jiangshi Weapon Chest".to_string(), 100.),   // 77.05 + 22.95 = 100
    ]
}

fn create_deva_loot_table() -> LootTable {
    vec![
        ("Small Golden Deva Soul Shield Chest".to_string(), 35.),
        ("Medium Golden Deva Soul Shield Chest".to_string(), 65.), // 35 + 30  = 65
        ("Deva Helm".to_string(), 69.),                            // 65 + 4 = 69
        ("Deva Aura".to_string(), 73.),                            // 69 + 4 = 73
        ("Deva Raiment".to_string(), 77.),                         // 73 + 4 = 77
        ("Ciderlands Golden Deva Card".to_string(), 77.05),        // 77 + 0.2 = 77.2
        ("Golden Deva Weapon Chest".to_string(), 100.),            // 77.2 + 22.8 = 100
    ]
}

fn create_kaari_loot_table() -> LootTable {
    vec![
        ("Small Kaari Soul Shield Chest".to_string(), 25.),
        ("Medium Kaari Soul Shield Chest".to_string(), 45.), // 25 + 20  = 45
        ("Valor Stone Chest".to_string(), 67.8),             // 45 + 22.8  = 67.8
        ("Kaari Mask".to_string(), 69.),                     // 67.8 + 4 = 69
        ("Kaari Googles".to_string(), 73.),                  // 69 + 4 = 73
        ("Kaari Suit".to_string(), 77.),                     // 73 + 4 = 77
        ("Moonwater Plains King Kaari Card".to_string(), 77.2), // 77 + 0.2 = 77.2
        ("Kaari Weapon Chest".to_string(), 100.),            // 77.2 + 20 = 100
    ]
}

fn create_profane_loot_table() -> LootTable {
    vec![
        ("Small Profane Soul Shield Chest".to_string(), 25.),
        ("Medium Profane Soul Shield Chest".to_string(), 45.), // 25 + 20  = 45
        ("Valor Stone Chest".to_string(), 66.),                // 45 + 21 = 66
        ("Profane Jinagshi Headpiece".to_string(), 70.),       // 66 + 4 = 70
        ("Profane Jinagshi Charm".to_string(), 74.),           // 70 + 4 = 74
        ("Profane Shroud".to_string(), 78.),                   // 74 + 4 = 78
        ("Moonwater Plains Profane Jiangshi Card".to_string(), 80.), // 78 + 2.0 = 80
        ("Profane Jiangshi Weapon Chest".to_string(), 100.),   // 80 + 20 = 100
    ]
}
