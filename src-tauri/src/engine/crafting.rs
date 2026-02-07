use crate::models::*;

pub fn execute_craft(
    first: &str,
    second: Option<&str>,
    state: &mut WorldState,
) -> ActionResult {
    // If no second item, list known recipes or prompt
    if second.is_none() {
        return list_recipes(first, state);
    }

    let second = second.unwrap();

    // Find items in player inventory matching these names
    let first_id = match find_inventory_item(first, state) {
        Some(id) => id,
        None => return craft_error(&format!("You don't have anything called '{}'.", first)),
    };
    let second_id = match find_inventory_item(second, state) {
        Some(id) => id,
        None => return craft_error(&format!("You don't have anything called '{}'.", second)),
    };

    if first_id == second_id {
        return craft_error("You can't combine an item with itself.");
    }

    // Check recipes
    let recipe = state.recipes.iter().position(|r| {
        r.inputs.contains(&first_id) && r.inputs.contains(&second_id) && r.inputs.len() == 2
    });

    match recipe {
        Some(idx) => {
            let output_id = state.recipes[idx].output.clone();
            let recipe_id = state.recipes[idx].id.clone();
            state.recipes[idx].discovered = true;

            // Remove inputs from inventory
            if let Some(pos) = state.player.inventory.iter().position(|x| *x == first_id) {
                state.player.inventory.remove(pos);
            }
            if let Some(pos) = state.player.inventory.iter().position(|x| *x == second_id) {
                state.player.inventory.remove(pos);
            }

            // Add output to inventory
            state.player.inventory.push(output_id.clone());

            let output_name = state
                .items
                .get(&output_id)
                .map(|i| i.name.clone())
                .unwrap_or_else(|| output_id.clone());
            let first_name = state
                .items
                .get(&first_id)
                .map(|i| i.name.clone())
                .unwrap_or(first_id);
            let second_name = state
                .items
                .get(&second_id)
                .map(|i| i.name.clone())
                .unwrap_or(second_id);

            // Add journal entry for crafting
            let journal_id = format!("craft_{}", recipe_id);
            if !state.journal.iter().any(|e| e.id == journal_id) {
                state.journal.push(JournalEntry {
                    id: journal_id,
                    category: JournalCategory::Item,
                    title: output_name.clone(),
                    content: format!("Crafted from {} and {}.", first_name, second_name),
                    discovered_turn: state.player.turns_elapsed,
                });
            }

            ActionResult {
                messages: vec![OutputLine {
                    text: format!(
                        "You combine {} and {} to create {}!",
                        first_name, second_name, output_name
                    ),
                    line_type: LineType::System,
                }],
                action_type: ActionType::ItemUsed {
                    item_name: output_name,
                    effect: "crafted".into(),
                },
                narrative_context: None,
                sound_cues: vec![SoundCue::ItemUse],
            }
        }
        None => {
            // Check if either item is part of a recipe (hint)
            let hint = state.recipes.iter().find(|r| {
                r.inputs.contains(&first_id) || r.inputs.contains(&second_id)
            });
            let msg = if let Some(h) = hint {
                format!("Those items don't combine. Hint: {}", h.hint)
            } else {
                "Those items can't be combined into anything useful.".to_string()
            };
            craft_error(&msg)
        }
    }
}

fn find_inventory_item(name: &str, state: &WorldState) -> Option<String> {
    let name_lower = name.to_lowercase();
    state.player.inventory.iter().find(|id| {
        let id_lower = id.to_lowercase();
        if id_lower == name_lower {
            return true;
        }
        if let Some(item) = state.items.get(*id) {
            let item_name = item.name.to_lowercase();
            item_name == name_lower
                || item_name.contains(&name_lower)
                || id_lower.contains(&name_lower)
        } else {
            false
        }
    }).cloned()
}

fn list_recipes(query: &str, state: &WorldState) -> ActionResult {
    let query_lower = query.to_lowercase();
    // Check if this is a single-word craft attempt -- show discovered recipes
    if query_lower == "recipes" || query_lower == "list" {
        let discovered: Vec<&CraftingRecipe> =
            state.recipes.iter().filter(|r| r.discovered).collect();
        if discovered.is_empty() {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "You haven't discovered any recipes yet. Try combining items!"
                        .to_string(),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![],
            };
        }
        let mut lines = vec![OutputLine {
            text: "--- Known Recipes ---".to_string(),
            line_type: LineType::System,
        }];
        for recipe in discovered {
            let input_names: Vec<String> = recipe
                .inputs
                .iter()
                .map(|id| {
                    state
                        .items
                        .get(id)
                        .map(|i| i.name.clone())
                        .unwrap_or_else(|| id.clone())
                })
                .collect();
            let output_name = state
                .items
                .get(&recipe.output)
                .map(|i| i.name.clone())
                .unwrap_or_else(|| recipe.output.clone());
            lines.push(OutputLine {
                text: format!(
                    "  {} + {} = {}",
                    input_names[0],
                    input_names.get(1).unwrap_or(&"?".to_string()),
                    output_name
                ),
                line_type: LineType::System,
            });
        }
        return ActionResult {
            messages: lines,
            action_type: ActionType::DisplayOnly,
            narrative_context: None,
            sound_cues: vec![],
        };
    }

    // Otherwise it's "craft <item>" without "with" -- prompt for second item
    ActionResult {
        messages: vec![OutputLine {
            text: format!(
                "Craft {} with what? Use: craft <item> with <item>",
                query
            ),
            line_type: LineType::System,
        }],
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
        sound_cues: vec![],
    }
}

fn craft_error(msg: &str) -> ActionResult {
    ActionResult {
        messages: vec![OutputLine {
            text: msg.to_string(),
            line_type: LineType::Error,
        }],
        action_type: ActionType::Error {
            message: msg.to_string(),
        },
        narrative_context: None,
        sound_cues: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    fn state_with_recipes() -> WorldState {
        let mut state = world_builder::build_thornhold();
        // Put inputs in player inventory
        state.player.inventory.push("torn_tapestry".into());
        state.player.inventory.push("quill_pen".into());
        state
    }

    #[test]
    fn craft_success() {
        let mut state = state_with_recipes();
        let result = execute_craft("torn tapestry", Some("quill pen"), &mut state);
        assert!(result.messages[0].text.contains("Makeshift Bandage"));
        assert!(state
            .player
            .inventory
            .contains(&"makeshift_bandage".to_string()));
        assert!(!state
            .player
            .inventory
            .contains(&"torn_tapestry".to_string()));
        assert!(!state.player.inventory.contains(&"quill_pen".to_string()));
        assert!(state.recipes.iter().any(|r| r.id == "makeshift_bandage" && r.discovered));
    }

    #[test]
    fn craft_missing_item() {
        let mut state = state_with_recipes();
        state.player.inventory.clear();
        let result = execute_craft("torn tapestry", Some("quill pen"), &mut state);
        assert!(result.messages[0].text.contains("don't have"));
    }

    #[test]
    fn craft_no_recipe() {
        let mut state = state_with_recipes();
        state.player.inventory.push("rusty_lantern".into());
        let result = execute_craft("torn tapestry", Some("rusty lantern"), &mut state);
        assert!(
            result.messages[0].text.contains("can't be combined")
                || result.messages[0].text.contains("Hint")
        );
    }

    #[test]
    fn craft_list_recipes() {
        let mut state = state_with_recipes();
        // Discover a recipe first
        let recipe_idx = state
            .recipes
            .iter()
            .position(|r| r.id == "makeshift_bandage")
            .unwrap();
        state.recipes[recipe_idx].discovered = true;
        let result = execute_craft("recipes", None, &mut state);
        assert!(result
            .messages
            .iter()
            .any(|m| m.text.contains("Torn Tapestry")));
    }

    #[test]
    fn craft_same_item() {
        let mut state = state_with_recipes();
        let result = execute_craft("torn tapestry", Some("torn tapestry"), &mut state);
        assert!(result.messages[0]
            .text
            .contains("can't combine an item with itself"));
    }

    #[test]
    fn craft_no_discovered_recipes() {
        let mut state = state_with_recipes();
        let result = execute_craft("recipes", None, &mut state);
        assert!(result.messages[0]
            .text
            .contains("haven't discovered any recipes"));
    }

    #[test]
    fn craft_prompt_for_second_item() {
        let mut state = state_with_recipes();
        let result = execute_craft("sword", None, &mut state);
        assert!(result.messages[0].text.contains("with what?"));
    }
}
