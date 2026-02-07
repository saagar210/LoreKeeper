use crate::models::{Direction, GameMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameCommand {
    Look(Option<String>),
    Go(Direction),
    Take(String),
    Drop(String),
    Use(String),
    Equip(String),
    Unequip(String),
    TalkTo(String),
    Attack(String),
    Flee,
    Inventory,
    Map,
    QuestLog,
    Journal,
    Craft(String, Option<String>),
    Secret(String),
    Help,
    Save(Option<String>),
    Load(Option<String>),
    Unknown(String),
}

fn strip_articles(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let filtered: Vec<&str> = words
        .into_iter()
        .filter(|w| !matches!(*w, "the" | "a" | "an" | "at" | "to"))
        .collect();
    filtered.join(" ")
}

pub fn parse(input: &str, game_mode: &GameMode) -> GameCommand {
    let cleaned = input.trim().to_lowercase();
    let cleaned = cleaned
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    if cleaned.is_empty() {
        return GameCommand::Unknown(String::new());
    }

    // In dialogue mode, most text is treated as dialogue response
    if let GameMode::InDialogue(_) = game_mode {
        match cleaned.as_str() {
            "leave" | "goodbye" | "bye" | "exit" | "quit" => {
                return GameCommand::Unknown("leave".to_string());
            }
            "inventory" | "inv" | "i" => return GameCommand::Inventory,
            "help" | "?" | "h" => return GameCommand::Help,
            _ => return GameCommand::Unknown(cleaned),
        }
    }

    // In combat mode, restrict commands
    if let GameMode::InCombat(_) = game_mode {
        let result = parse_command(&cleaned);
        match &result {
            GameCommand::Attack(_)
            | GameCommand::Flee
            | GameCommand::Use(_)
            | GameCommand::Inventory
            | GameCommand::Help => return result,
            _ => {
                return GameCommand::Unknown(
                    "You're in combat! Attack, use an item, or flee!".to_string(),
                )
            }
        }
    }

    parse_command(&cleaned)
}

fn parse_command(cleaned: &str) -> GameCommand {
    let parts: Vec<&str> = cleaned.splitn(2, ' ').collect();
    let verb = parts[0];
    let rest = if parts.len() > 1 { parts[1] } else { "" };

    // Direction shortcuts
    if let Some(dir) = Direction::parse(verb) {
        return GameCommand::Go(dir);
    }

    match verb {
        // Movement
        "go" | "move" | "walk" | "head" => {
            let target = strip_articles(rest);
            if let Some(dir) = Direction::parse(&target) {
                GameCommand::Go(dir)
            } else if target.is_empty() {
                GameCommand::Unknown("Go where?".to_string())
            } else {
                GameCommand::Unknown(format!("Unknown direction: {}", target))
            }
        }

        // Look
        "look" | "l" | "examine" | "inspect" | "x" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Look(None)
            } else {
                GameCommand::Look(Some(target))
            }
        }

        // Take
        "take" | "get" | "grab" | "pick" => {
            let target = if verb == "pick" {
                strip_articles(rest.strip_prefix("up").unwrap_or(rest))
            } else {
                strip_articles(rest)
            };
            if target.is_empty() {
                GameCommand::Unknown("Take what?".to_string())
            } else {
                GameCommand::Take(target)
            }
        }

        // Drop
        "drop" | "discard" | "throw" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Drop what?".to_string())
            } else {
                GameCommand::Drop(target)
            }
        }

        // Use
        "use" | "drink" | "eat" | "read" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Use what?".to_string())
            } else {
                GameCommand::Use(target)
            }
        }

        // Equip
        "equip" | "wield" | "wear" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Equip what?".to_string())
            } else {
                GameCommand::Equip(target)
            }
        }

        // Unequip
        "unequip" | "remove" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Unequip what?".to_string())
            } else {
                GameCommand::Unequip(target)
            }
        }

        // Talk
        "talk" | "speak" | "ask" | "chat" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Talk to whom?".to_string())
            } else {
                GameCommand::TalkTo(target)
            }
        }

        // Attack
        "attack" | "fight" | "hit" | "kill" | "strike" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Attack what?".to_string())
            } else {
                GameCommand::Attack(target)
            }
        }

        // Flee
        "flee" | "run" | "escape" => GameCommand::Flee,

        // Meta
        "inventory" | "inv" | "i" => GameCommand::Inventory,
        "map" | "m" => GameCommand::Map,
        "quests" | "journal" | "quest" => GameCommand::QuestLog,
        "codex" | "notes" | "lore" => GameCommand::Journal,
        "help" | "?" => GameCommand::Help,

        // Save/Load
        "save" => {
            if rest.is_empty() {
                GameCommand::Save(None)
            } else {
                GameCommand::Save(Some(rest.to_string()))
            }
        }
        "load" => {
            if rest.is_empty() {
                GameCommand::Load(None)
            } else {
                GameCommand::Load(Some(rest.to_string()))
            }
        }

        // Crafting
        "craft" | "combine" | "mix" => {
            let target = strip_articles(rest);
            if target.is_empty() {
                GameCommand::Unknown("Craft what?".to_string())
            } else if let Some(with_idx) = target.find(" with ") {
                let first = target[..with_idx].trim().to_string();
                let second = target[with_idx + 6..].trim().to_string();
                GameCommand::Craft(first, Some(second))
            } else if let Some(and_idx) = target.find(" and ") {
                let first = target[..and_idx].trim().to_string();
                let second = target[and_idx + 5..].trim().to_string();
                GameCommand::Craft(first, Some(second))
            } else {
                GameCommand::Craft(target, None)
            }
        }

        "xyzzy" | "plugh" | "abracadabra" | "sesame" | "opensesame" => {
            GameCommand::Secret(verb.to_string())
        }

        _ => GameCommand::Unknown(cleaned.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exploring() -> GameMode {
        GameMode::Exploring
    }

    #[test]
    fn parse_directions() {
        assert_eq!(
            parse("north", &exploring()),
            GameCommand::Go(Direction::North)
        );
        assert_eq!(
            parse("n", &exploring()),
            GameCommand::Go(Direction::North)
        );
        assert_eq!(
            parse("go south", &exploring()),
            GameCommand::Go(Direction::South)
        );
        assert_eq!(
            parse("go east", &exploring()),
            GameCommand::Go(Direction::East)
        );
        assert_eq!(
            parse("  WEST  ", &exploring()),
            GameCommand::Go(Direction::West)
        );
        assert_eq!(parse("u", &exploring()), GameCommand::Go(Direction::Up));
        assert_eq!(parse("d", &exploring()), GameCommand::Go(Direction::Down));
    }

    #[test]
    fn parse_look() {
        assert_eq!(parse("look", &exploring()), GameCommand::Look(None));
        assert_eq!(parse("l", &exploring()), GameCommand::Look(None));
        assert_eq!(
            parse("look at the sword", &exploring()),
            GameCommand::Look(Some("sword".to_string()))
        );
        assert_eq!(
            parse("examine rusty lantern", &exploring()),
            GameCommand::Look(Some("rusty lantern".to_string()))
        );
        assert_eq!(
            parse("x sword", &exploring()),
            GameCommand::Look(Some("sword".to_string()))
        );
    }

    #[test]
    fn parse_take_drop() {
        assert_eq!(
            parse("take sword", &exploring()),
            GameCommand::Take("sword".to_string())
        );
        assert_eq!(
            parse("get the rusty lantern", &exploring()),
            GameCommand::Take("rusty lantern".to_string())
        );
        assert_eq!(
            parse("pick up the key", &exploring()),
            GameCommand::Take("key".to_string())
        );
        assert_eq!(
            parse("drop sword", &exploring()),
            GameCommand::Drop("sword".to_string())
        );
    }

    #[test]
    fn parse_use_equip() {
        assert_eq!(
            parse("use health potion", &exploring()),
            GameCommand::Use("health potion".to_string())
        );
        assert_eq!(
            parse("drink potion", &exploring()),
            GameCommand::Use("potion".to_string())
        );
        assert_eq!(
            parse("equip sword", &exploring()),
            GameCommand::Equip("sword".to_string())
        );
        assert_eq!(
            parse("unequip shield", &exploring()),
            GameCommand::Unequip("shield".to_string())
        );
    }

    #[test]
    fn parse_interaction() {
        assert_eq!(
            parse("talk to merchant", &exploring()),
            GameCommand::TalkTo("merchant".to_string())
        );
        assert_eq!(
            parse("attack goblin", &exploring()),
            GameCommand::Attack("goblin".to_string())
        );
        assert_eq!(parse("flee", &exploring()), GameCommand::Flee);
        assert_eq!(parse("run", &exploring()), GameCommand::Flee);
    }

    #[test]
    fn parse_meta() {
        assert_eq!(parse("inventory", &exploring()), GameCommand::Inventory);
        assert_eq!(parse("inv", &exploring()), GameCommand::Inventory);
        assert_eq!(parse("i", &exploring()), GameCommand::Inventory);
        assert_eq!(parse("map", &exploring()), GameCommand::Map);
        assert_eq!(parse("quests", &exploring()), GameCommand::QuestLog);
        assert_eq!(parse("help", &exploring()), GameCommand::Help);
    }

    #[test]
    fn parse_save_load() {
        assert_eq!(parse("save", &exploring()), GameCommand::Save(None));
        assert_eq!(
            parse("save my game", &exploring()),
            GameCommand::Save(Some("my game".to_string()))
        );
        assert_eq!(parse("load", &exploring()), GameCommand::Load(None));
        assert_eq!(
            parse("load slot1", &exploring()),
            GameCommand::Load(Some("slot1".to_string()))
        );
    }

    #[test]
    fn parse_combat_mode_restrictions() {
        let combat = GameMode::InCombat("goblin".to_string());
        assert_eq!(
            parse("attack goblin", &combat),
            GameCommand::Attack("goblin".to_string())
        );
        assert_eq!(parse("flee", &combat), GameCommand::Flee);
        assert_eq!(parse("inventory", &combat), GameCommand::Inventory);
        // Non-combat commands are restricted
        if let GameCommand::Unknown(msg) = parse("go north", &combat) {
            assert!(msg.contains("combat"));
        } else {
            panic!("Expected combat restriction");
        }
    }

    #[test]
    fn parse_dialogue_mode() {
        let dialogue = GameMode::InDialogue("merchant".to_string());
        assert_eq!(parse("inventory", &dialogue), GameCommand::Inventory);
        assert_eq!(parse("help", &dialogue), GameCommand::Help);
        // Regular text becomes dialogue
        assert_eq!(
            parse("hello there", &dialogue),
            GameCommand::Unknown("hello there".to_string())
        );
        assert_eq!(
            parse("leave", &dialogue),
            GameCommand::Unknown("leave".to_string())
        );
    }

    #[test]
    fn parse_empty_input() {
        assert_eq!(
            parse("", &exploring()),
            GameCommand::Unknown(String::new())
        );
        assert_eq!(
            parse("   ", &exploring()),
            GameCommand::Unknown(String::new())
        );
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!(
            parse("LOOK AT THE SWORD", &exploring()),
            GameCommand::Look(Some("sword".to_string()))
        );
        assert_eq!(
            parse("Go North", &exploring()),
            GameCommand::Go(Direction::North)
        );
    }

    #[test]
    fn parse_craft() {
        assert_eq!(
            parse("craft sword with gem", &exploring()),
            GameCommand::Craft("sword".into(), Some("gem".into()))
        );
        assert_eq!(
            parse("combine bone and silver", &exploring()),
            GameCommand::Craft("bone".into(), Some("silver".into()))
        );
        assert_eq!(
            parse("craft", &exploring()),
            GameCommand::Unknown("Craft what?".to_string())
        );
        assert_eq!(
            parse("craft recipes", &exploring()),
            GameCommand::Craft("recipes".into(), None)
        );
    }

    #[test]
    fn parse_secret_commands() {
        assert_eq!(parse("xyzzy", &exploring()), GameCommand::Secret("xyzzy".into()));
        assert_eq!(parse("plugh", &exploring()), GameCommand::Secret("plugh".into()));
        assert_eq!(parse("abracadabra", &exploring()), GameCommand::Secret("abracadabra".into()));
        assert_eq!(parse("sesame", &exploring()), GameCommand::Secret("sesame".into()));
    }

    #[test]
    fn parse_codex() {
        assert_eq!(parse("codex", &exploring()), GameCommand::Journal);
        assert_eq!(parse("notes", &exploring()), GameCommand::Journal);
        assert_eq!(parse("lore", &exploring()), GameCommand::Journal);
    }
}
