use std::collections::HashMap;

use crate::models::*;

pub fn build_thornhold() -> WorldState {
    WorldState {
        locations: build_locations(),
        items: build_items(),
        npcs: build_npcs(),
        quests: build_quests(),
        events: build_events(),
        recipes: build_recipes(),
        initialized: true,
        ..Default::default()
    }
}

fn build_locations() -> HashMap<String, Location> {
    let mut locs = HashMap::new();

    locs.insert("courtyard".into(), Location {
        id: "courtyard".into(),
        name: "The Courtyard".into(),
        description: "Crumbling stone walls surround a desolate courtyard. Weeds push through cracks in the flagstones. A cold wind carries whispers of those who once gathered here.".into(),
        items: vec!["rusty_lantern".into(), "merchant_journal".into()],
        npcs: vec!["merchant_ghost".into()],
        exits: HashMap::from([
            (Direction::East, "great_hall".into()),
            (Direction::South, "barracks".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: true,
        discovered_secrets: vec![],
        ambient_mood: Mood::Peaceful,
        examine_details: Some("The flagstones bear scorch marks from an ancient battle. Faded carvings on the walls depict merchants trading goods. A broken fountain stands in the center, its basin cracked and dry.".into()),
        revisit_description: Some("The courtyard is as bleak as before. The cold wind still whispers.".into()),
    });

    locs.insert("great_hall".into(), Location {
        id: "great_hall".into(),
        name: "The Great Hall".into(),
        description: "Once magnificent, the great hall now stands in shadow. Tattered banners hang from the vaulted ceiling. A massive fireplace dominates the north wall, cold and dark.".into(),
        items: vec!["torn_tapestry".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::West, "courtyard".into()),
            (Direction::East, "library".into()),
            (Direction::South, "kitchen".into()),
            (Direction::Up, "tower_apex".into()),
        ]),
        locked_exits: HashMap::from([
            (Direction::East, "library_key".into()),
        ]),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Mysterious,
        examine_details: Some("The banners bear the crest of House Thornhold — a tower wreathed in thorns. Claw marks gouge the stone floor near the fireplace. A faint draft comes from behind the eastern wall.".into()),
        revisit_description: Some("The great hall looms in familiar shadow. The cold fireplace watches like a dark eye.".into()),
    });

    locs.insert("tower_apex".into(), Location {
        id: "tower_apex".into(),
        name: "The Tower Apex".into(),
        description: "The highest point of Thornhold. Wind howls through broken windows. The view stretches endlessly — dark forests, distant mountains, and the ruins below.".into(),
        items: vec!["old_spyglass".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::Down, "great_hall".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Tense,
        examine_details: Some("From here you can see the entire ruin spread below. Scratches on the window frame suggest someone — or something — tried to climb in. A weathervane creaks overhead, pointing eternally north.".into()),
        revisit_description: None,
    });

    locs.insert("library".into(), Location {
        id: "library".into(),
        name: "The Library".into(),
        description: "Shelves of rotting books line the walls. The air is thick with dust and the smell of ancient parchment. Knowledge lingers here, waiting to be found.".into(),
        items: vec!["sacred_scroll".into(), "dusty_tome".into(), "quill_pen".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::West, "great_hall".into()),
            (Direction::South, "chapel".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Mysterious,
        examine_details: Some("Many books have been deliberately torn apart. One shelf holds a collection of sealed scrolls. The dust on the floor shows no footprints — you are the first visitor in ages.".into()),
        revisit_description: Some("The library's dusty silence greets you once more.".into()),
    });

    locs.insert("barracks".into(), Location {
        id: "barracks".into(),
        name: "The Barracks".into(),
        description: "Rows of collapsed bunks fill this room. Rusted weapons hang on racks. Something moves in the shadows — bones scraping against stone.".into(),
        items: vec!["iron_shield".into()],
        npcs: vec!["skeletal_guard".into()],
        exits: HashMap::from([
            (Direction::North, "courtyard".into()),
            (Direction::East, "kitchen".into()),
            (Direction::South, "armory".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Tense,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("kitchen".into(), Location {
        id: "kitchen".into(),
        name: "The Kitchen".into(),
        description: "A vast kitchen, long abandoned. Pots hang from hooks, thick with grime. Something rustles behind the overturned table — a pair of bright eyes watches you.".into(),
        items: vec!["stale_bread".into(), "health_potion".into()],
        npcs: vec!["gristle_rat".into()],
        exits: HashMap::from([
            (Direction::North, "great_hall".into()),
            (Direction::West, "barracks".into()),
            (Direction::East, "chapel".into()),
            (Direction::South, "cellar_entrance".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Peaceful,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("chapel".into(), Location {
        id: "chapel".into(),
        name: "The Chapel".into(),
        description: "Stained glass windows cast colored shadows across stone pews. An altar stands at the far end, still bearing offerings from ages past. A sense of peace lingers here.".into(),
        items: vec!["silver_chalice".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::North, "library".into()),
            (Direction::West, "kitchen".into()),
            (Direction::South, "crypt_passage".into()),
        ]),
        locked_exits: HashMap::from([
            (Direction::South, "chapel_seal".into()),
        ]),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Sacred,
        examine_details: Some("The stained glass depicts the founding of Thornhold. The altar bears scratch marks, as if something tried to deface it. A faint warmth radiates from the stone.".into()),
        revisit_description: Some("The chapel's colored light washes over you again. The altar waits patiently.".into()),
    });

    locs.insert("armory".into(), Location {
        id: "armory".into(),
        name: "The Armory".into(),
        description: "Weapon racks and armor stands fill this room. Most are rusted beyond use, but a few pieces remain serviceable. The air smells of oil and old metal.".into(),
        items: vec!["short_sword".into(), "leather_armor".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::North, "barracks".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Tense,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("cellar_entrance".into(), Location {
        id: "cellar_entrance".into(),
        name: "Cellar Entrance".into(),
        description: "Stone steps descend into darkness. The air grows cold and damp. Water drips somewhere below, echoing in the silence.".into(),
        items: vec!["torch".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::North, "kitchen".into()),
            (Direction::Down, "wine_cellar".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Dark,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("wine_cellar".into(), Location {
        id: "wine_cellar".into(),
        name: "The Wine Cellar".into(),
        description: "Rows of dusty barrels and empty bottles line the walls. The smell of old wine mixes with damp earth. A narrow passage leads further down.".into(),
        items: vec!["empty_bottle".into(), "cellar_cheese".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::Up, "cellar_entrance".into()),
            (Direction::Down, "crypt_passage".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Dark,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("crypt_passage".into(), Location {
        id: "crypt_passage".into(),
        name: "The Crypt Passage".into(),
        description: "A narrow corridor of ancient stone. Bones protrude from the walls. The temperature drops with each step. An unnatural cold seeps into your very bones.".into(),
        items: vec!["bone_fragment".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::Up, "wine_cellar".into()),
            (Direction::Down, "deep_chamber".into()),
            (Direction::North, "chapel".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Dark,
        examine_details: None,
        revisit_description: None,
    });

    locs.insert("deep_chamber".into(), Location {
        id: "deep_chamber".into(),
        name: "The Deep Chamber".into(),
        description: "A vast underground chamber, lit by phosphorescent fungi. Ancient runes cover the walls. The air thrums with a power both ancient and terrible.".into(),
        items: vec!["ancient_amulet".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::Up, "crypt_passage".into()),
            (Direction::Down, "final_sanctum".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Dangerous,
        examine_details: Some("The runes on the walls shift when you look away. The fungi pulse in a rhythm like a heartbeat. Chains embedded in the far wall have been snapped, links scattered across the floor.".into()),
        revisit_description: None,
    });

    locs.insert("final_sanctum".into(), Location {
        id: "final_sanctum".into(),
        name: "The Final Sanctum".into(),
        description: "The heart of Thornhold. A circular chamber pulsing with eldritch light. At its center stands a figure, ancient and terrible, bound in chains of fading magic.".into(),
        items: vec!["mysterious_orb".into()],
        npcs: vec!["the_forgotten_one".into()],
        exits: HashMap::from([
            (Direction::Up, "deep_chamber".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Dangerous,
        examine_details: Some("The chains binding the figure are inscribed with names — perhaps those who placed them. The eldritch light emanates from a crack in the floor. The air tastes of copper and ozone.".into()),
        revisit_description: None,
    });

    locs.insert("hidden_vault".into(), Location {
        id: "hidden_vault".into(),
        name: "The Hidden Vault".into(),
        description: "A secret chamber behind the walls. Dust motes dance in a shaft of light from a crack in the ceiling. Ancient treasures and forgotten relics line the shelves.".into(),
        items: vec!["vault_amulet".into()],
        npcs: vec![],
        exits: HashMap::from([
            (Direction::Up, "great_hall".into()),
        ]),
        locked_exits: HashMap::new(),
        visited: false,
        discovered_secrets: vec![],
        ambient_mood: Mood::Mysterious,
        examine_details: Some("The shelves hold trinkets from across the ages — a child's toy, a soldier's medal, a lover's locket. Each tells a story of Thornhold's past.".into()),
        revisit_description: Some("The hidden vault is as you left it. The treasures gleam in the dim light.".into()),
    });

    locs
}

fn build_items() -> HashMap<String, Item> {
    let mut items = HashMap::new();

    // Weapons
    items.insert("short_sword".into(), Item {
        id: "short_sword".into(),
        name: "Short Sword".into(),
        description: "A well-balanced blade, still sharp despite its age. It feels good in your hand.".into(),
        item_type: ItemType::Weapon,
        modifier: Some(StatModifier { attack: 4, defense: 0, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("Forged by the smiths of Thornhold in its golden age. The maker's mark — a tiny tower — is etched near the hilt.".into()),
    });

    items.insert("rusty_dagger".into(), Item {
        id: "rusty_dagger".into(),
        name: "Rusty Dagger".into(),
        description: "A pitted dagger, once belonging to The Warden. Still wickedly sharp.".into(),
        item_type: ItemType::Weapon,
        modifier: Some(StatModifier { attack: 2, defense: 0, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    // Armor
    items.insert("leather_armor".into(), Item {
        id: "leather_armor".into(),
        name: "Leather Armor".into(),
        description: "Cracked but serviceable leather armor. It still offers some protection.".into(),
        item_type: ItemType::Armor,
        modifier: Some(StatModifier { attack: 0, defense: 3, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("iron_shield".into(), Item {
        id: "iron_shield".into(),
        name: "Iron Shield".into(),
        description: "A heavy iron shield, dented but functional. The crest has been scratched away.".into(),
        item_type: ItemType::Armor,
        modifier: Some(StatModifier { attack: 0, defense: 4, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    // Consumables
    items.insert("health_potion".into(), Item {
        id: "health_potion".into(),
        name: "Health Potion".into(),
        description: "A small vial of crimson liquid. It glows faintly with healing magic.".into(),
        item_type: ItemType::Consumable,
        modifier: Some(StatModifier { attack: 0, defense: 0, health: 30 }),
        usable: true,
        consumable: true,
        key_id: None,
        lore: None,
    });

    items.insert("stale_bread".into(), Item {
        id: "stale_bread".into(),
        name: "Stale Bread".into(),
        description: "A loaf of bread, hard as stone. But food is food in a place like this.".into(),
        item_type: ItemType::Consumable,
        modifier: Some(StatModifier { attack: 0, defense: 0, health: 10 }),
        usable: true,
        consumable: true,
        key_id: None,
        lore: None,
    });

    items.insert("cellar_cheese".into(), Item {
        id: "cellar_cheese".into(),
        name: "Aged Cellar Cheese".into(),
        description: "A wheel of surprisingly well-preserved cheese. Pungent but appealing.".into(),
        item_type: ItemType::Consumable,
        modifier: Some(StatModifier { attack: 0, defense: 0, health: 5 }),
        usable: true,
        consumable: true,
        key_id: None,
        lore: None,
    });

    // Keys
    items.insert("library_key".into(), Item {
        id: "library_key".into(),
        name: "Library Key".into(),
        description: "An ornate brass key. The head is shaped like an open book.".into(),
        item_type: ItemType::Key,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: Some("library".into()),
        lore: None,
    });

    // Scrolls
    items.insert("sacred_scroll".into(), Item {
        id: "sacred_scroll".into(),
        name: "Sacred Scroll".into(),
        description: "An ancient scroll covered in sacred text. It radiates a faint warmth. Perhaps it could be used at the Chapel.".into(),
        item_type: ItemType::Scroll,
        modifier: None,
        usable: true,
        consumable: true,
        key_id: None,
        lore: Some("Written by the last priest of Thornhold before the fall. The ink shimmers with divine power that has endured centuries.".into()),
    });

    // Quest items
    items.insert("merchant_journal".into(), Item {
        id: "merchant_journal".into(),
        name: "Merchant's Journal".into(),
        description: "A leather-bound journal filled with a merchant's final entries. The last page begs for the journal to be placed on the Chapel altar.".into(),
        item_type: ItemType::Quest,
        modifier: None,
        usable: true,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("silver_chalice".into(), Item {
        id: "silver_chalice".into(),
        name: "Silver Chalice".into(),
        description: "A tarnished silver chalice. Despite its age, it still holds a certain reverence.".into(),
        item_type: ItemType::Quest,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("ancient_amulet".into(), Item {
        id: "ancient_amulet".into(),
        name: "Ancient Amulet".into(),
        description: "A heavy amulet of dark metal. Strange symbols pulse with a faint inner light.".into(),
        item_type: ItemType::Quest,
        modifier: Some(StatModifier { attack: 2, defense: 2, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("One of the sealing artifacts used to bind The Forgotten One. Its power has weakened over the centuries but still resonates with protective magic.".into()),
    });

    items.insert("mysterious_orb".into(), Item {
        id: "mysterious_orb".into(),
        name: "Mysterious Orb".into(),
        description: "A sphere of pure darkness that seems to absorb all light. It hums with power.".into(),
        item_type: ItemType::Quest,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("The concentrated essence of The Forgotten One's power. Holding it grants visions of a world before Thornhold, when gods walked the earth.".into()),
    });

    // Miscellaneous
    items.insert("rusty_lantern".into(), Item {
        id: "rusty_lantern".into(),
        name: "Rusty Lantern".into(),
        description: "A battered lantern. The oil is long gone, but it might be useful.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("torn_tapestry".into(), Item {
        id: "torn_tapestry".into(),
        name: "Torn Tapestry".into(),
        description: "A shredded tapestry depicting the fall of Thornhold. Only fragments of the story remain.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("old_spyglass".into(), Item {
        id: "old_spyglass".into(),
        name: "Old Spyglass".into(),
        description: "A brass spyglass. Through it, you can see the world as it once was — or perhaps as it could be.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("quill_pen".into(), Item {
        id: "quill_pen".into(),
        name: "Quill Pen".into(),
        description: "An ancient quill pen. The nib is still stained with ink.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("dusty_tome".into(), Item {
        id: "dusty_tome".into(),
        name: "Dusty Tome".into(),
        description: "A thick book bound in cracked leather. The pages speak of the history of Thornhold and the creature imprisoned beneath.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("Chronicles the founding of Thornhold as a prison for an ancient being. The final chapter, written in a shaking hand, warns that the binding weakens with each passing century.".into()),
    });

    items.insert("empty_bottle".into(), Item {
        id: "empty_bottle".into(),
        name: "Empty Bottle".into(),
        description: "A dusty wine bottle. The label has long since faded.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("bone_fragment".into(), Item {
        id: "bone_fragment".into(),
        name: "Bone Fragment".into(),
        description: "A fragment of ancient bone. It feels unnaturally cold to the touch.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    items.insert("torch".into(), Item {
        id: "torch".into(),
        name: "Torch".into(),
        description: "A simple wooden torch wrapped in oil-soaked cloth. Ready to be lit.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: None,
        usable: false,
        consumable: false,
        key_id: None,
        lore: None,
    });

    // Crafted items
    items.insert("makeshift_bandage".into(), Item {
        id: "makeshift_bandage".into(),
        name: "Makeshift Bandage".into(),
        description: "A crude bandage fashioned from torn tapestry cloth, bound with a quill pen.".into(),
        item_type: ItemType::Consumable,
        modifier: Some(StatModifier { attack: 0, defense: 0, health: 20 }),
        usable: true,
        consumable: true,
        key_id: None,
        lore: Some("Resourcefulness in desperate times. The tapestry of Thornhold's history now serves to heal.".into()),
    });

    items.insert("lit_lantern".into(), Item {
        id: "lit_lantern".into(),
        name: "Lit Lantern".into(),
        description: "The rusty lantern now burns with a steady flame, pushing back the darkness.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: Some(StatModifier { attack: 0, defense: 1, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("Even the oldest tools can serve again when given purpose.".into()),
    });

    items.insert("bone_talisman".into(), Item {
        id: "bone_talisman".into(),
        name: "Bone Talisman".into(),
        description: "A fragment of ancient bone set into a silver chalice base. It radiates protective energy.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: Some(StatModifier { attack: 1, defense: 3, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("The silver purifies while the bone remembers. Together they ward against the darkness below.".into()),
    });

    items.insert("vault_amulet".into(), Item {
        id: "vault_amulet".into(),
        name: "Vault Amulet".into(),
        description: "A perfectly preserved amulet of deep blue crystal. It thrums with a steady, protective pulse.".into(),
        item_type: ItemType::Miscellaneous,
        modifier: Some(StatModifier { attack: 3, defense: 5, health: 0 }),
        usable: false,
        consumable: false,
        key_id: None,
        lore: Some("One of the original warding stones of Thornhold. Only those who know the old words can find where it is hidden.".into()),
    });

    items
}

fn build_recipes() -> Vec<CraftingRecipe> {
    vec![
        CraftingRecipe {
            id: "makeshift_bandage".into(),
            inputs: vec!["torn_tapestry".into(), "quill_pen".into()],
            output: "makeshift_bandage".into(),
            hint: "Something torn could bind a wound with the right tool...".into(),
            discovered: false,
        },
        CraftingRecipe {
            id: "lantern_torch".into(),
            inputs: vec!["rusty_lantern".into(), "torch".into()],
            output: "lit_lantern".into(),
            hint: "A lantern needs a flame...".into(),
            discovered: false,
        },
        CraftingRecipe {
            id: "bone_talisman".into(),
            inputs: vec!["bone_fragment".into(), "silver_chalice".into()],
            output: "bone_talisman".into(),
            hint: "Bone and silver have warding properties...".into(),
            discovered: false,
        },
    ]
}

fn build_npcs() -> HashMap<String, Npc> {
    let mut npcs = HashMap::new();

    npcs.insert("merchant_ghost".into(), Npc {
        id: "merchant_ghost".into(),
        name: "The Dead Merchant".into(),
        description: "A translucent figure in merchant's robes. His eyes hold centuries of sorrow. He wrings spectral hands, unable to find peace.".into(),
        personality_seed: "Melancholic and formal. Speaks in old-fashioned manner. Desperately wants his journal delivered to the Chapel altar.".into(),
        dialogue_state: DialogueState::Greeting,
        hostile: false,
        health: 1,
        max_health: 1,
        attack: 0,
        defense: 0,
        items: vec![],
        quest_giver: Some("merchants_unfinished_business".into()),
        examine_text: Some("His robes bear the insignia of the Thornhold Merchant Guild. A heavy ledger hangs from a spectral chain at his belt. His expression carries centuries of regret.".into()),
        relationship: 0,
        memory: vec![],
    });

    npcs.insert("gristle_rat".into(), Npc {
        id: "gristle_rat".into(),
        name: "Gristle".into(),
        description: "An unusually large rat with intelligent eyes. It chatters and squeaks, somehow making itself understood. It seems desperate for something.".into(),
        personality_seed: "Nervous and helpful. Obsessed with cheese. Speaks in short, excited sentences. Knows secrets about the cellar.".into(),
        dialogue_state: DialogueState::Greeting,
        hostile: false,
        health: 5,
        max_health: 5,
        attack: 1,
        defense: 0,
        items: vec![],
        quest_giver: Some("rats_request".into()),
        examine_text: None,
        relationship: 0,
        memory: vec![],
    });

    npcs.insert("skeletal_guard".into(), Npc {
        id: "skeletal_guard".into(),
        name: "Skeletal Guard".into(),
        description: "An animated skeleton in rusted armor. It grips a notched sword and stands vigil over the barracks, as it has for centuries.".into(),
        personality_seed: "Silent and hostile. Attacks on sight. Bound by ancient duty.".into(),
        dialogue_state: DialogueState::Hostile,
        hostile: true,
        health: 25,
        max_health: 25,
        attack: 7,
        defense: 4,
        items: vec!["library_key".into()],
        quest_giver: None,
        examine_text: None,
        relationship: 0,
        memory: vec![],
    });

    npcs.insert("the_warden".into(), Npc {
        id: "the_warden".into(),
        name: "The Warden".into(),
        description: "A massive, hooded figure wreathed in shadow. Eyes like dying embers peer from beneath the hood. It radiates menace and ancient purpose.".into(),
        personality_seed: "Menacing and philosophical. Questions why you trespass. Offers cryptic warnings about what lies below.".into(),
        dialogue_state: DialogueState::Hostile,
        hostile: true,
        health: 40,
        max_health: 40,
        attack: 10,
        defense: 6,
        items: vec!["rusty_dagger".into()],
        quest_giver: None,
        examine_text: None,
        relationship: 0,
        memory: vec![],
    });

    npcs.insert("the_forgotten_one".into(), Npc {
        id: "the_forgotten_one".into(),
        name: "The Forgotten One".into(),
        description: "An ancient being of terrible power, bound in chains of fading magic. Its form shifts between human and something else entirely. It speaks with the weight of millennia.".into(),
        personality_seed: "Ancient, weary, and surprisingly reasonable. Offers negotiation if the player has proven worthy. Can be fought or reasoned with.".into(),
        dialogue_state: DialogueState::Greeting,
        hostile: false,
        health: 60,
        max_health: 60,
        attack: 15,
        defense: 8,
        items: vec![],
        quest_giver: Some("the_final_confrontation".into()),
        examine_text: Some("Its form flickers between shapes — now a crowned king, now a beast of shadow, now something that has no name. The chains binding it glow faintly where they touch its shifting form.".into()),
        relationship: 0,
        memory: vec![],
    });

    npcs
}

fn build_quests() -> HashMap<String, Quest> {
    let mut quests = HashMap::new();

    quests.insert("the_lost_key".into(), Quest {
        id: "the_lost_key".into(),
        name: "The Lost Key".into(),
        description: "The Skeletal Guard in the barracks holds a key. Defeat it to claim the Library Key.".into(),
        giver: "skeletal_guard".into(),
        objective: QuestObjective::KillNpc("skeletal_guard".into()),
        reward: vec![],
        completed: false,
        active: true,
        completed_turn: None,
    });

    quests.insert("rats_request".into(), Quest {
        id: "rats_request".into(),
        name: "The Rat's Request".into(),
        description: "Gristle the rat desperately wants cheese from the Wine Cellar. Find the Aged Cellar Cheese and bring it back.".into(),
        giver: "gristle_rat".into(),
        objective: QuestObjective::FetchItem("cellar_cheese".into()),
        reward: vec!["health_potion".into()],
        completed: false,
        active: false,
        completed_turn: None,
    });

    quests.insert("merchants_unfinished_business".into(), Quest {
        id: "merchants_unfinished_business".into(),
        name: "The Merchant's Unfinished Business".into(),
        description: "The ghost merchant begs you to take his journal to the Chapel altar. Use the journal at the Chapel to complete his final wish.".into(),
        giver: "merchant_ghost".into(),
        objective: QuestObjective::FetchItem("merchant_journal".into()),
        reward: vec![],
        completed: false,
        active: false,
        completed_turn: None,
    });

    quests.insert("the_final_confrontation".into(), Quest {
        id: "the_final_confrontation".into(),
        name: "The Final Confrontation".into(),
        description: "Face The Forgotten One in the Final Sanctum. You may fight or negotiate — but only the worthy may negotiate.".into(),
        giver: "the_forgotten_one".into(),
        objective: QuestObjective::ReachLocation("final_sanctum".into()),
        reward: vec![],
        completed: false,
        active: false,
        completed_turn: None,
    });

    quests
}

fn build_events() -> Vec<GameEvent> {
    vec![
        // Crypt passage damage (repeating)
        GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Damage(5),
            one_shot: false,
            fired: false,
            location_id: "crypt_passage".into(),
        },
        // Deep Chamber: spawn The Warden on first visit
        GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::SpawnNpc("the_warden".into()),
            one_shot: true,
            fired: false,
            location_id: "deep_chamber".into(),
        },
        // Ancient amulet pickup message
        GameEvent {
            trigger: EventTrigger::OnTake("ancient_amulet".into()),
            action: EventAction::Message("The amulet pulses with warmth as you touch it. You feel a connection to something ancient.".into()),
            one_shot: true,
            fired: false,
            location_id: "deep_chamber".into(),
        },
        // Sacred scroll at Chapel unlocks passage to Crypt
        GameEvent {
            trigger: EventTrigger::OnUse("sacred_scroll".into()),
            action: EventAction::Unlock(Direction::South),
            one_shot: true,
            fired: false,
            location_id: "chapel".into(),
        },
        GameEvent {
            trigger: EventTrigger::OnUse("sacred_scroll".into()),
            action: EventAction::Message("The scroll dissolves into light. Ancient words echo through the chapel. The floor trembles — a hidden passage opens downward.".into()),
            one_shot: true,
            fired: false,
            location_id: "chapel".into(),
        },
        // Merchant journal at Chapel
        GameEvent {
            trigger: EventTrigger::OnUse("merchant_journal".into()),
            action: EventAction::SetQuestFlag("merchant_quest_complete".into()),
            one_shot: true,
            fired: false,
            location_id: "chapel".into(),
        },
        GameEvent {
            trigger: EventTrigger::OnUse("merchant_journal".into()),
            action: EventAction::Message("You place the journal on the altar. It glows briefly, then fades. Somewhere, a spirit finds peace. You feel blessed.".into()),
            one_shot: true,
            fired: false,
            location_id: "chapel".into(),
        },
        // Armory trap on first visit
        GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Damage(10),
            one_shot: true,
            fired: false,
            location_id: "armory".into(),
        },
        GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Message("A blade swings from the shadows! You barely dodge, but it catches your side.".into()),
            one_shot: true,
            fired: false,
            location_id: "armory".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_is_initialized() {
        let state = build_thornhold();
        assert!(state.initialized);
        assert_eq!(state.player.location, "courtyard");
        assert_eq!(state.game_mode, GameMode::Exploring);
    }

    #[test]
    fn all_locations_exist() {
        let state = build_thornhold();
        assert_eq!(state.locations.len(), 14);
        let expected = vec![
            "courtyard", "great_hall", "tower_apex", "library", "barracks",
            "kitchen", "chapel", "armory", "cellar_entrance", "wine_cellar",
            "crypt_passage", "deep_chamber", "final_sanctum", "hidden_vault",
        ];
        for id in expected {
            assert!(state.locations.contains_key(id), "Missing location: {}", id);
        }
    }

    #[test]
    fn exits_are_bidirectional() {
        let state = build_thornhold();
        // hidden_vault is a secret room reachable only via the "plugh" command,
        // so its exit to great_hall is intentionally one-way at world build time.
        let secret_exits: std::collections::HashSet<(&str, &str)> =
            [("hidden_vault", "great_hall")].into_iter().collect();
        for (loc_id, loc) in &state.locations {
            for (dir, dest_id) in &loc.exits {
                if secret_exits.contains(&(loc_id.as_str(), dest_id.as_str())) {
                    continue;
                }
                let dest = state.locations.get(dest_id).unwrap_or_else(|| {
                    panic!("Exit from {} -> {} leads to non-existent location {}", loc_id, dir, dest_id)
                });
                let reverse = dir.opposite();
                assert!(
                    dest.exits.contains_key(&reverse),
                    "Exit {} -> {} ({}) has no return path {} -> {} ({})",
                    loc_id, dest_id, dir, dest_id, loc_id, reverse
                );
                assert_eq!(
                    dest.exits.get(&reverse).unwrap(),
                    loc_id,
                    "Return path from {} ({}) doesn't lead back to {}",
                    dest_id, reverse, loc_id
                );
            }
        }
    }

    #[test]
    fn all_items_in_locations_exist() {
        let state = build_thornhold();
        for (loc_id, loc) in &state.locations {
            for item_id in &loc.items {
                assert!(
                    state.items.contains_key(item_id),
                    "Location {} references non-existent item {}",
                    loc_id, item_id
                );
            }
        }
    }

    #[test]
    fn all_npcs_in_locations_exist() {
        let state = build_thornhold();
        for (loc_id, loc) in &state.locations {
            for npc_id in &loc.npcs {
                assert!(
                    state.npcs.contains_key(npc_id),
                    "Location {} references non-existent NPC {}",
                    loc_id, npc_id
                );
            }
        }
    }

    #[test]
    fn quest_givers_exist() {
        let state = build_thornhold();
        for (quest_id, quest) in &state.quests {
            assert!(
                state.npcs.contains_key(&quest.giver),
                "Quest {} references non-existent giver {}",
                quest_id, quest.giver
            );
        }
    }

    #[test]
    fn quest_objectives_are_valid() {
        let state = build_thornhold();
        for (quest_id, quest) in &state.quests {
            match &quest.objective {
                QuestObjective::FetchItem(item_id) => {
                    assert!(
                        state.items.contains_key(item_id),
                        "Quest {} has FetchItem objective for non-existent item {}",
                        quest_id, item_id
                    );
                }
                QuestObjective::KillNpc(npc_id) => {
                    assert!(
                        state.npcs.contains_key(npc_id),
                        "Quest {} has KillNpc objective for non-existent NPC {}",
                        quest_id, npc_id
                    );
                }
                QuestObjective::ReachLocation(loc_id) => {
                    assert!(
                        state.locations.contains_key(loc_id),
                        "Quest {} has ReachLocation objective for non-existent location {}",
                        quest_id, loc_id
                    );
                }
            }
        }
    }

    #[test]
    fn locked_exits_have_keys() {
        let state = build_thornhold();
        // Collect exits that are unlocked by events (not keys)
        let event_unlocked: std::collections::HashSet<(&str, &Direction)> = state
            .events
            .iter()
            .filter_map(|e| {
                if let EventAction::Unlock(dir) = &e.action {
                    Some((e.location_id.as_str(), dir))
                } else {
                    None
                }
            })
            .collect();

        for (loc_id, loc) in &state.locations {
            for (dir, key_id) in &loc.locked_exits {
                // Skip exits that are event-unlocked (no physical key needed)
                if event_unlocked.contains(&(loc_id.as_str(), dir)) {
                    continue;
                }
                // Key must exist somewhere in the world (items or NPC inventory)
                let in_items = state.items.contains_key(key_id);
                let in_locations = state.locations.values().any(|l| l.items.contains(key_id));
                let in_npc_inventory = state.npcs.values().any(|n| n.items.contains(key_id));
                assert!(
                    in_items && (in_locations || in_npc_inventory),
                    "Locked exit {} -> {} requires key {} but it doesn't exist in the world",
                    loc_id, dir, key_id
                );
            }
        }
    }

    #[test]
    fn player_starting_location_exists() {
        let state = build_thornhold();
        assert!(state.locations.contains_key(&state.player.location));
    }
}
