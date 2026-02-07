use crate::models::*;

pub fn process_events(
    trigger: &EventTrigger,
    location_id: &str,
    state: &mut WorldState,
) -> Vec<OutputLine> {
    let mut messages = Vec::new();

    let mut actions_to_apply: Vec<EventAction> = Vec::new();
    let mut events_to_mark_fired: Vec<usize> = Vec::new();

    for (idx, event) in state.events.iter().enumerate() {
        if event.location_id != location_id {
            continue;
        }
        if event.one_shot && event.fired {
            continue;
        }
        if &event.trigger != trigger {
            continue;
        }
        actions_to_apply.push(event.action.clone());
        if event.one_shot {
            events_to_mark_fired.push(idx);
        }
    }

    for idx in events_to_mark_fired {
        state.events[idx].fired = true;
    }

    for action in actions_to_apply {
        match action {
            EventAction::Damage(amount) => {
                state.player.health = (state.player.health - amount).max(0);
                messages.push(OutputLine {
                    text: crate::engine::templates::describe_event_damage(amount),
                    line_type: LineType::Combat,
                });
            }
            EventAction::SpawnNpc(npc_id) => {
                if let Some(loc) = state.locations.get_mut(location_id) {
                    if !loc.npcs.contains(&npc_id) {
                        loc.npcs.push(npc_id.clone());
                    }
                }
                messages.push(OutputLine {
                    text: "A presence manifests before you...".to_string(),
                    line_type: LineType::Narration,
                });
            }
            EventAction::Unlock(direction) => {
                if let Some(loc) = state.locations.get_mut(location_id) {
                    loc.locked_exits.remove(&direction);
                }
                messages.push(OutputLine {
                    text: format!("A passage {} has been revealed!", direction.display_name()),
                    line_type: LineType::System,
                });
            }
            EventAction::Message(msg) => {
                messages.push(OutputLine {
                    text: crate::engine::templates::describe_event_message(&msg),
                    line_type: LineType::Narration,
                });
            }
            EventAction::GiveItem(item_id) => {
                if state.player.inventory.len() < state.player.max_inventory {
                    state.player.inventory.push(item_id.clone());
                    if let Some(item) = state.items.get(&item_id) {
                        messages.push(OutputLine {
                            text: format!("You received: {}", item.name),
                            line_type: LineType::System,
                        });
                    }
                } else if let Some(item) = state.items.get(&item_id) {
                    messages.push(OutputLine {
                        text: format!(
                            "Your inventory is full! The {} falls to the ground.",
                            item.name
                        ),
                        line_type: LineType::System,
                    });
                    // Drop item in current location instead
                    if let Some(loc) =
                        state.locations.get_mut(&state.player.location)
                    {
                        loc.items.push(item_id.clone());
                    }
                }
            }
            EventAction::SetQuestFlag(flag) => {
                state.player.quest_flags.insert(flag.clone(), true);
            }
        }
    }

    messages.retain(|m| !m.text.is_empty());
    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_test_state() -> WorldState {
        let mut state = WorldState::default();
        state.locations.insert(
            "test_room".into(),
            Location {
                id: "test_room".into(),
                name: "Test".into(),
                description: "Test room.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::new(),
                locked_exits: HashMap::from([(Direction::North, "key1".into())]),
                visited: false,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );
        state.player.location = "test_room".into();
        state
    }

    #[test]
    fn process_damage_event() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Damage(10),
            one_shot: false,
            fired: false,
            location_id: "test_room".into(),
        });

        let msgs = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(state.player.health, 90);
        assert!(!msgs.is_empty());
    }

    #[test]
    fn one_shot_event_only_fires_once() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Message("Trap!".into()),
            one_shot: true,
            fired: false,
            location_id: "test_room".into(),
        });

        let msgs1 = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(msgs1.len(), 1);
        let msgs2 = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(msgs2.len(), 0);
    }

    #[test]
    fn unlock_event_removes_locked_exit() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnUse("scroll".into()),
            action: EventAction::Unlock(Direction::North),
            one_shot: true,
            fired: false,
            location_id: "test_room".into(),
        });

        let trigger = EventTrigger::OnUse("scroll".into());
        process_events(&trigger, "test_room", &mut state);
        let loc = state.locations.get("test_room").unwrap();
        assert!(!loc.locked_exits.contains_key(&Direction::North));
    }
}
