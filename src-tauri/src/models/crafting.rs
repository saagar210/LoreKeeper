use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftingRecipe {
    pub id: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub hint: String,
    #[serde(default)]
    pub discovered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_serde_roundtrip() {
        let recipe = CraftingRecipe {
            id: "test".into(),
            inputs: vec!["a".into(), "b".into()],
            output: "c".into(),
            hint: "Combine a and b".into(),
            discovered: false,
        };
        let json = serde_json::to_string(&recipe).unwrap();
        let parsed: CraftingRecipe = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "test");
        assert_eq!(parsed.inputs.len(), 2);
    }
}
