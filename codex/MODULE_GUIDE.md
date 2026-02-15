# LoreKeeper Module Creation Guide

**Version:** 1.0
**Status:** Complete

Create custom adventure modules to extend LoreKeeper with new worlds, quests, NPCs, and items.

---

## Quick Start

1. **Create a module JSON file** with your custom world data
2. **Validate** using the in-game Map Editor
3. **Load** from the Module Select screen
4. **Play** your custom adventure!

---

## Module Structure

Every module is a single JSON file with this structure:

```json
{
  "name": "My Custom Adventure",
  "version": "1.0.0",
  "description": "A unique dungeon experience",
  "author": "Your Name",
  "locations": [],
  "npcs": [],
  "items": [],
  "quests": [],
  "recipes": []
}
```

---

## Locations

Define rooms/areas in your adventure:

```json
{
  "id": "entrance_chamber",
  "name": "Entrance Chamber",
  "description": "A grand entrance with towering columns.",
  "ambientMood": "mysterious",
  "items": ["torch", "rusty_key"],
  "npcs": ["guardian_npc"],
  "exits": {
    "north": "main_hall",
    "east": "treasure_room"
  },
  "lockedExits": {
    "south": "iron_key"
  },
  "examineDetails": "The columns bear ancient carvings.",
  "revisitDescription": "You've been here before."
}
```

**Fields:**
- `id` (string): Unique identifier (lowercase, underscores)
- `name` (string): Display name
- `description` (string): Initial room description
- `ambientMood` (string): "peaceful", "mysterious", "tense", "dark", "sacred"
- `items` (array): Item IDs in this location
- `npcs` (array): NPC IDs in this location
- `exits` (object): Map of direction → location ID
- `lockedExits` (object): Map of direction → key item ID required to pass
- `examineDetails` (string): Extra description when examining
- `revisitDescription` (string): Text shown on returning

**Directions:** `north`, `south`, `east`, `west`, `up`, `down`

---

## NPCs

Define non-player characters:

```json
{
  "id": "ancient_sage",
  "name": "Ancient Sage",
  "description": "An old wizard with knowing eyes.",
  "personalitySeed": "wise and cryptic",
  "hostile": false,
  "canFight": false,
  "health": 50,
  "maxHealth": 50,
  "attack": 0,
  "defense": 5,
  "items": ["staff_of_power"],
  "questGiver": "retrieve_artifact",
  "examineText": "The wizard's robes shimmer with magic.",
  "relationship": 0
}
```

**Fields:**
- `id` (string): Unique identifier
- `name` (string): Display name
- `description` (string): First impression text
- `personalitySeed` (string): Personality descriptor (used for dialogue)
- `hostile` (boolean): Attacks on sight?
- `canFight` (boolean): Can be fought?
- `health` (number): Current health
- `maxHealth` (number): Maximum health
- `attack` (number): Damage dealt in combat
- `defense` (number): Damage reduction
- `items` (array): Items the NPC carries
- `questGiver` (string): Quest ID they give (optional)
- `examineText` (string): Examine description
- `relationship` (number): Starting relationship value

---

## Items

Define objects that can be picked up and used:

```json
{
  "id": "healing_potion",
  "name": "Healing Potion",
  "description": "A vial of shimmering red liquid.",
  "itemType": "consumable",
  "modifier": {
    "attack": 0,
    "defense": 0,
    "health": 50
  },
  "usable": true,
  "consumable": true,
  "keyId": null,
  "lore": "Brewed from rare herbs."
}
```

**Fields:**
- `id` (string): Unique identifier
- `name` (string): Display name
- `description` (string): Examine text
- `itemType` (string): Type of item
  - "weapon" - increases attack
  - "armor" - increases defense
  - "consumable" - used and consumed
  - "key" - unlocks doors
  - "miscellaneous" - quest items, lore items
- `modifier` (object): Stat changes
  - `attack` (number): Attack bonus
  - `defense` (number): Defense bonus
  - `health` (number): Health modification
- `usable` (boolean): Can be "used" in game?
- `consumable` (boolean): Disappears after use?
- `keyId` (string): Which lock does this key open? (for keys only)
- `lore` (string): Backstory or flavor text

**Item Types:**
- Weapons: Used to increase attack damage
- Armor: Equipped for defense bonus
- Consumables: Provide health or temporary benefits
- Keys: Unlock specific doors (via `lockedExits`)
- Miscellaneous: Story items, quest objectives, decorations

---

## Quests

Define tasks for players:

```json
{
  "id": "find_amulet",
  "name": "Retrieve the Ancient Amulet",
  "description": "The sage needs an ancient amulet from the deep cavern.",
  "giver": "ancient_sage",
  "objective": {
    "type": "FetchItem",
    "itemId": "ancient_amulet"
  },
  "reward": ["reward_item_1", "gold_coin"],
  "active": true
}
```

**Objective Types:**
- `FetchItem`: Collect a specific item
- `KillNpc`: Defeat a specific NPC
- `ReachLocation`: Visit a specific location

**Fields:**
- `id` (string): Unique quest identifier
- `name` (string): Quest title
- `description` (string): Quest description
- `giver` (string): NPC ID who gives the quest
- `objective` (object): What the player must do
- `reward` (array): Item IDs given on completion
- `active` (boolean): Is quest available at game start?

---

## Recipes

Define craftable items:

```json
{
  "id": "iron_sword_recipe",
  "name": "Iron Sword",
  "ingredients": ["iron_ore", "wood_handle"],
  "result": "iron_sword",
  "discovered": false
}
```

**Fields:**
- `id` (string): Unique recipe identifier
- `name` (string): Recipe name
- `ingredients` (array): Item IDs required to craft
- `result` (string): Item ID created
- `discovered` (boolean): Is recipe known at start?

---

## Validation Rules

Your module must follow these rules to be valid:

1. **All location exits must reference existing locations**
   - ❌ Bad: `"north": "nonexistent_room"`
   - ✅ Good: `"north": "main_hall"` (if main_hall exists)

2. **All locked exits must reference existing key items**
   - ❌ Bad: `"locked_exits": {"east": "no_such_key"}`
   - ✅ Good: `"locked_exits": {"east": "iron_key"}` (if iron_key exists in items)

3. **All NPCs in locations must exist in the npcs array**
   - ❌ Bad: Location references `"npcs": ["guardian"]` but guardian not defined
   - ✅ Good: guardian NPC exists in npcs array

4. **All items in locations must exist in the items array**
   - ❌ Bad: Location references `"items": ["sword"]` but sword not defined
   - ✅ Good: sword item exists in items array

5. **All quest givers must be defined NPCs**
   - ❌ Bad: `"giver": "unknown_npc"`
   - ✅ Good: `"giver": "ancient_sage"` (if ancient_sage exists)

6. **Recipe ingredients and results must be valid items**
   - ❌ Bad: `"ingredients": ["nonexistent_ore"]`
   - ✅ Good: `"ingredients": ["iron_ore"]` (if defined in items)

7. **No circular location references**
   - Rooms can connect back, but should form valid paths

---

## Complete Example Module

```json
{
  "name": "The Lost Crypt",
  "version": "1.0.0",
  "description": "A small underground crypt with treasures and puzzles.",
  "author": "Game Creator",
  "locations": [
    {
      "id": "crypt_entrance",
      "name": "Crypt Entrance",
      "description": "Ancient stone doors, slightly ajar. Dust falls from the ceiling.",
      "ambientMood": "dark",
      "items": ["torch"],
      "npcs": [],
      "exits": {
        "north": "burial_chamber",
        "south": "outside"
      },
      "lockedExits": {},
      "examineDetails": "Stone carved with protective runes."
    },
    {
      "id": "burial_chamber",
      "name": "Burial Chamber",
      "description": "Elaborate stone coffins line the walls. Gold glints in the darkness.",
      "ambientMood": "mysterious",
      "items": ["gold_amulet", "ancient_scroll"],
      "npcs": ["tomb_guardian"],
      "exits": {
        "south": "crypt_entrance",
        "east": "treasure_vault"
      },
      "lockedExits": {
        "east": "vault_key"
      },
      "examineDetails": "The coffins bear the names of ancient kings."
    },
    {
      "id": "treasure_vault",
      "name": "Treasure Vault",
      "description": "Gold coins and gems cover the floor. Piles of wealth beyond measure.",
      "ambientMood": "sacred",
      "items": ["gold_coin", "diamond"],
      "npcs": [],
      "exits": {
        "west": "burial_chamber"
      },
      "lockedExits": {},
      "examineDetails": "The treasure here is worth a fortune."
    }
  ],
  "npcs": [
    {
      "id": "tomb_guardian",
      "name": "Tomb Guardian",
      "description": "A skeletal figure in ancient armor.",
      "personalitySeed": "hostile and protective",
      "hostile": true,
      "canFight": true,
      "health": 30,
      "maxHealth": 30,
      "attack": 8,
      "defense": 4,
      "items": ["vault_key"],
      "questGiver": null,
      "examineText": "Its eyes glow with an eerie light.",
      "relationship": 0
    }
  ],
  "items": [
    {
      "id": "torch",
      "name": "Torch",
      "description": "A wooden torch with burning flames.",
      "itemType": "miscellaneous",
      "modifier": {"attack": 0, "defense": 0, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": null,
      "lore": "Provides light in darkness."
    },
    {
      "id": "vault_key",
      "name": "Vault Key",
      "description": "An ornate golden key.",
      "itemType": "key",
      "modifier": {"attack": 0, "defense": 0, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": "treasure_vault",
      "lore": "Unlocks the treasure vault."
    },
    {
      "id": "gold_amulet",
      "name": "Gold Amulet",
      "description": "A beautiful amulet of pure gold.",
      "itemType": "miscellaneous",
      "modifier": {"attack": 2, "defense": 2, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": null,
      "lore": "A royal treasure from ages past."
    },
    {
      "id": "gold_coin",
      "name": "Gold Coin",
      "description": "A shiny gold coin.",
      "itemType": "miscellaneous",
      "modifier": {"attack": 0, "defense": 0, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": null,
      "lore": "Worth a small fortune."
    },
    {
      "id": "diamond",
      "name": "Diamond",
      "description": "A flawless diamond that sparkles brilliantly.",
      "itemType": "miscellaneous",
      "modifier": {"attack": 0, "defense": 0, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": null,
      "lore": "One of the rarest gems in the world."
    },
    {
      "id": "ancient_scroll",
      "name": "Ancient Scroll",
      "description": "A rolled parchment covered in dust.",
      "itemType": "miscellaneous",
      "modifier": {"attack": 0, "defense": 0, "health": 0},
      "usable": false,
      "consumable": false,
      "keyId": null,
      "lore": "Contains forgotten knowledge."
    }
  ],
  "quests": [],
  "recipes": []
}
```

---

## Testing Your Module

1. **Open the Map Editor** from the title screen
2. **Paste your JSON** into the editor
3. **Click Validate** to check for errors
4. **Fix any errors** shown in the validation report
5. **Export** to save your module
6. **Load from Module Select** to play it

---

## Tips & Best Practices

- **Keep location descriptions vivid and immersive**
- **Balance challenge** - mix easy and hard NPCs
- **Create clear paths** - ensure players can navigate
- **Use locked exits** for a sense of progression
- **Provide quest hooks** through NPC dialogue
- **Add flavor lore** to items and locations
- **Test thoroughly** - play through your own module
- **Limit size** - complex modules with 50+ locations may slow loading

---

## Troubleshooting

**Module won't load:**
- Check JSON syntax (use a JSON validator)
- Ensure all referenced IDs exist
- Verify location exits form valid paths

**Game crashes when loading:**
- Simplify the module (reduce locations/NPCs)
- Check for circular references
- Ensure all text fields are strings

**Items/NPCs don't appear:**
- Check location's items/npcs arrays reference correct IDs
- Verify IDs match exactly (case-sensitive)

---

## Future Enhancements

Planned features for module system:
- Custom starting locations
- Custom player stats
- Conditional quests (based on player choices)
- Module-specific achievements
- Custom dialogue trees
- Animated descriptions

---

**Ready to create?** Start with the example above and customize each section. Happy adventuring!
