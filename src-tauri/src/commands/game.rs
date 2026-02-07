use tauri::{Emitter, Manager, State};
use tokio::sync::mpsc;

use crate::engine::{
    achievement_checker,
    dungeon_generator::{self, DungeonConfig},
    executor, hints, parser, world_builder,
};
use crate::models::*;
use crate::narrative::narrator::{self, NarrativeEvent};
use crate::persistence::state::{DbState, GameState, SettingsState};
use crate::persistence::stats;

#[tauri::command]
pub fn initialize_game(game_state: State<GameState>) -> Result<CommandResponse, String> {
    let mut state = game_state.0.lock().map_err(|e| e.to_string())?;

    if !state.initialized {
        *state = world_builder::build_thornhold();

        // Generate procedural dungeon wing
        let difficulty_base = match state.difficulty {
            Difficulty::Easy => 3,
            Difficulty::Normal => 5,
            Difficulty::Hard => 8,
        };
        dungeon_generator::generate_dungeon(
            &DungeonConfig {
                entry_location: "armory".to_string(),
                entry_direction: Direction::Down,
                depth: 5,
                difficulty_base,
            },
            &mut state,
        );
    }

    let loc = state.locations.get(&state.player.location).cloned();
    let messages = if let Some(location) = loc {
        let mut msgs = vec![
            OutputLine {
                text: "Welcome to The Depths of Thornhold.".into(),
                line_type: LineType::System,
            },
            OutputLine {
                text: "Type 'help' for a list of commands.".into(),
                line_type: LineType::System,
            },
            OutputLine {
                text: String::new(),
                line_type: LineType::System,
            },
        ];
        let look_lines =
            crate::engine::templates::describe_location(&location, &state.items, &state.npcs, true);
        msgs.extend(look_lines.into_iter().map(|text| OutputLine {
            text,
            line_type: LineType::Narration,
        }));
        msgs
    } else {
        vec![OutputLine {
            text: "Error: Starting location not found.".into(),
            line_type: LineType::Error,
        }]
    };

    Ok(CommandResponse {
        messages,
        world_state: state.clone(),
        sound_cues: vec![],
    })
}

#[tauri::command]
pub fn new_game(
    app: tauri::AppHandle,
    game_state: State<GameState>,
    settings_state: State<SettingsState>,
) -> Result<CommandResponse, String> {
    let mut state = game_state.0.lock().map_err(|e| e.to_string())?;
    *state = world_builder::build_thornhold();

    // Copy difficulty from settings
    if let Ok(settings) = settings_state.0.lock() {
        state.difficulty = settings.difficulty;
    }

    // Generate procedural dungeon wing
    let difficulty_base = match state.difficulty {
        Difficulty::Easy => 3,
        Difficulty::Normal => 5,
        Difficulty::Hard => 8,
    };
    dungeon_generator::generate_dungeon(
        &DungeonConfig {
            entry_location: "armory".to_string(),
            entry_direction: Direction::Down,
            depth: 5,
            difficulty_base,
        },
        &mut state,
    );

    // Track games_started stat
    if let Some(db) = app.try_state::<DbState>() {
        if let Ok(conn) = db.0.lock() {
            let _ = stats::increment_stat(&conn, "games_started", 1);
        }
    }

    let loc = state.locations.get(&state.player.location).cloned();
    let messages = if let Some(location) = loc {
        let mut msgs = vec![
            OutputLine {
                text: "A new adventure begins...".into(),
                line_type: LineType::System,
            },
            OutputLine {
                text: String::new(),
                line_type: LineType::System,
            },
        ];
        let look_lines =
            crate::engine::templates::describe_location(&location, &state.items, &state.npcs, true);
        msgs.extend(look_lines.into_iter().map(|text| OutputLine {
            text,
            line_type: LineType::Narration,
        }));
        msgs
    } else {
        vec![OutputLine {
            text: "Error: Starting location not found.".into(),
            line_type: LineType::Error,
        }]
    };

    Ok(CommandResponse {
        messages,
        world_state: state.clone(),
        sound_cues: vec![],
    })
}

#[tauri::command]
pub async fn process_command(
    input: String,
    app: tauri::AppHandle,
    game_state: State<'_, GameState>,
    settings_state: State<'_, SettingsState>,
) -> Result<CommandResponse, String> {
    let (mut messages, world_state, narrative_ctx, settings, dialogue_llm_context);

    {
        let mut state = game_state.0.lock().map_err(|e| e.to_string())?;
        let command = parser::parse(&input, &state.game_mode);

        // Block commands when game is over (except save/load)
        if matches!(state.game_mode, GameMode::GameOver(_))
            && !matches!(command, parser::GameCommand::Save(_) | parser::GameCommand::Load(_))
        {
            return Ok(CommandResponse {
                messages: vec![OutputLine {
                    text: "Your adventure has ended. Load a save or start a new game.".into(),
                    line_type: LineType::System,
                }],
                world_state: state.clone(),
        sound_cues: vec![],
            });
        }

        // Handle save/load specially
        match &command {
            parser::GameCommand::Save(slot_name) => {
                let slot = slot_name
                    .clone()
                    .unwrap_or_else(|| "quicksave".to_string());
                let db_state = app.state::<crate::persistence::state::DbState>();
                let db = db_state.0.lock().map_err(|e| format!("{}", e))?;
                crate::persistence::save_load::save_game(&db, &slot, &state)?;
                return Ok(CommandResponse {
                    messages: vec![OutputLine {
                        text: format!("Game saved to '{}'.", slot),
                        line_type: LineType::System,
                    }],
                    world_state: state.clone(),
        sound_cues: vec![],
                });
            }
            parser::GameCommand::Load(slot_name) => {
                let slot = slot_name
                    .clone()
                    .unwrap_or_else(|| "quicksave".to_string());
                let db_state = app.state::<crate::persistence::state::DbState>();
                let db = db_state.0.lock().map_err(|e| format!("{}", e))?;
                let loaded = crate::persistence::save_load::load_game(&db, &slot)?;
                *state = loaded;
                let loc = state.locations.get(&state.player.location).cloned();
                let mut msgs = vec![OutputLine {
                    text: format!("Game loaded from '{}'.", slot),
                    line_type: LineType::System,
                }];
                if let Some(location) = loc {
                    let look_lines = crate::engine::templates::describe_location(
                        &location,
                        &state.items,
                        &state.npcs,
                        false,
                    );
                    msgs.extend(look_lines.into_iter().map(|text| OutputLine {
                        text,
                        line_type: LineType::Narration,
                    }));
                }
                return Ok(CommandResponse {
                    messages: msgs,
                    world_state: state.clone(),
        sound_cues: vec![],
                });
            }
            _ => {}
        }

        // Record command in log
        let log_entry = CommandLogEntry {
            turn: state.player.turns_elapsed,
            input: input.clone(),
            location: state.player.location.clone(),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
        };
        state.command_log.push(log_entry);

        let prev_visited_count = state.player.visited_locations.len();

        let result = executor::execute(command, &mut state);
        messages = result.messages;

        // Append contextual hint for new players
        if let Some(hint_text) = hints::get_contextual_hint(&state) {
            messages.push(OutputLine {
                text: format!("[Hint] {}", hint_text),
                line_type: LineType::System,
            });
        }

        // Track stats
        if let Some(db) = app.try_state::<DbState>() {
            if let Ok(conn) = db.0.lock() {
                let _ = stats::increment_stat(&conn, "commands_entered", 1);

                // Room explored (first visit)
                if state.player.visited_locations.len() > prev_visited_count {
                    let _ = stats::increment_stat(&conn, "rooms_explored", 1);
                }

                match &result.action_type {
                    ActionType::CombatVictory { .. } => {
                        let _ = stats::increment_stat(&conn, "enemies_defeated", 1);
                    }
                    ActionType::ItemTaken { .. } => {
                        let _ = stats::increment_stat(&conn, "items_collected", 1);
                    }
                    ActionType::QuestCompleted { .. } => {
                        let _ = stats::increment_stat(&conn, "quests_completed", 1);
                    }
                    ActionType::PlayerDeath => {
                        let _ = stats::increment_stat(&conn, "deaths", 1);
                    }
                    _ => {}
                }
            }
        }

        // Check achievements
        let newly_earned = achievement_checker::check_achievements(&state, &result.action_type);
        if !newly_earned.is_empty() {
            if let Some(db) = app.try_state::<DbState>() {
                if let Ok(conn) = db.0.lock() {
                    for ach_id in &newly_earned {
                        if !crate::persistence::achievements::is_unlocked(&conn, ach_id) {
                            let _ =
                                crate::persistence::achievements::unlock_achievement(&conn, ach_id);
                            if let Some(ach) = crate::models::achievement::all_achievements()
                                .into_iter()
                                .find(|a| a.id == *ach_id)
                            {
                                messages.push(OutputLine {
                                    text: format!(
                                        "[Achievement Unlocked] {} - {}",
                                        ach.name, ach.description
                                    ),
                                    line_type: LineType::System,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Save playthrough on game over
        if matches!(state.game_mode, GameMode::GameOver(_)) {
            if let Some(db) = app.try_state::<DbState>() {
                if let Ok(conn) = db.0.lock() {
                    let ending = match &state.game_mode {
                        GameMode::GameOver(e) => Some(format!("{:?}", e)),
                        _ => None,
                    };
                    let quests_done =
                        state.quests.values().filter(|q| q.completed).count() as i32;
                    let log_json =
                        serde_json::to_string(&state.command_log).unwrap_or_default();
                    let now = chrono::Utc::now().to_rfc3339();
                    let _ = conn.execute(
                        "INSERT INTO playthroughs (started_at, ended_at, ending_type, turns_taken, quests_completed, enemies_defeated, command_log) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        rusqlite::params![
                            now,
                            now,
                            ending,
                            state.player.turns_elapsed as i32,
                            quests_done,
                            0,
                            log_json,
                        ],
                    );
                }
            }
        }

        narrative_ctx = result.narrative_context;
        // Store last narrative context for retry
        if narrative_ctx.is_some() {
            state.last_narrative_context = narrative_ctx.clone();
        }

        // Detect free-form dialogue for LLM narration
        dialogue_llm_context = if let GameMode::InDialogue(ref npc_id) = state.game_mode {
            let npc_id = npc_id.clone();
            if let ActionType::NpcDialogue {
                ref dialogue_text, ..
            } = result.action_type
            {
                // Only route through LLM for free-form text, not quest mechanic results
                let is_quest_mechanic = matches!(
                    dialogue_text.as_str(),
                    "hostile" | "dead" | "declined"
                );
                if !is_quest_mechanic {
                    // Record user input in dialogue history
                    state.dialogue_history.push(DialogueHistoryEntry {
                        role: "user".to_string(),
                        text: input.clone(),
                    });
                    // Clone NPC data needed for LLM context
                    state.npcs.get(&npc_id).map(|npc| {
                        (
                            npc.name.clone(),
                            npc.personality_seed.clone(),
                            input.clone(),
                            npc.relationship,
                            npc.memory.clone(),
                            state
                                .dialogue_history
                                .iter()
                                .map(|e| (e.role.clone(), e.text.clone()))
                                .collect::<Vec<_>>(),
                        )
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        world_state = state.clone();
        settings = settings_state.0.lock().map_err(|e| e.to_string())?.clone();
    }

    // Spawn narrative generation in background
    if narrative_ctx.is_some() && settings.ollama_enabled {
        let (tx, mut rx) = mpsc::channel::<NarrativeEvent>(32);
        let state_clone = world_state.clone();
        let settings_clone = settings.clone();
        let ctx_clone = narrative_ctx.clone();

        tauri::async_runtime::spawn(async move {
            narrator::narrate(&ctx_clone, &state_clone, &settings_clone, &tx).await;
            drop(tx);
        });

        // Forward narrative events to frontend via Tauri events
        let app_clone2 = app.clone();
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                let _ = app_clone2.emit("narrative-event", &event);
            }
        });
    }

    // Spawn LLM dialogue narration for free-form NPC dialogue
    if let Some((npc_name, personality_seed, dialogue_text, relationship, memory, history)) =
        dialogue_llm_context
    {
        if settings.ollama_enabled {
            let (tx, mut rx) = mpsc::channel::<NarrativeEvent>(32);
            let settings_clone = settings.clone();

            tauri::async_runtime::spawn(async move {
                narrator::narrate_dialogue(
                    &npc_name,
                    &personality_seed,
                    &dialogue_text,
                    &settings_clone,
                    relationship,
                    &memory,
                    &history,
                    &tx,
                )
                .await;
                drop(tx);
            });

            let app_clone3 = app.clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    let _ = app_clone3.emit("narrative-event", &event);
                }
            });
        }
    }

    Ok(CommandResponse {
        messages,
        world_state,
        sound_cues: vec![],
    })
}
