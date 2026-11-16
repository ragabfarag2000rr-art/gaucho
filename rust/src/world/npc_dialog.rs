use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct NpcDialogOption {
    pub text: String,
    pub action: String,
    pub map: u32,
    pub x: i32,
    pub y: i32,
    pub amount: i32,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct NpcDialog {
    pub greeting: String,
    pub options: Vec<NpcDialogOption>,
}

pub fn load_npc_dialogs(dir: &Path) -> HashMap<u32, NpcDialog> {
    let path = dir.join("npc_dialogs.json");
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("parse {}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
        Err(e) => panic!("read {}: {e}", path.display()),
    }
}
