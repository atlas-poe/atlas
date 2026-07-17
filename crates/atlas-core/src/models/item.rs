use serde::{Deserialize, Serialize};

/// PoE2 item rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ItemRarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

/// A property displayed on an item (e.g., "Adds 10-20 Physical Damage")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperty {
    pub name: String,
    /// (display_text, augmentation_type)
    pub values: Vec<(String, i32)>,
    pub display_mode: u32,
    #[serde(rename = "type")]
    pub prop_type: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

/// Socket information for an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSocket {
    pub group: u32,
    /// PoE2: always "W"
    #[serde(rename = "attr")]
    pub attribute: String,
}

/// A PoE2 item from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    // Identity
    pub name: String,
    #[serde(rename = "typeLine")]
    pub type_line: String,
    pub base_type: String,
    pub rarity: ItemRarity,
    pub ilvl: u32,
    pub identified: bool,
    pub verified: bool,

    // Dimensions
    pub w: u32,
    pub h: u32,
    pub icon: String,

    // Stack info (for currency, fragments, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<u32>,

    // Properties and requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<ItemProperty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<ItemProperty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notable_properties: Vec<ItemProperty>,

    // Sockets (PoE2 uses "W" for all sockets)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gem_sockets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sockets: Vec<ItemSocket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub socketed_items: Vec<Item>,

    // Mods
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicit_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explicit_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub crafted_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enchant_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rune_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fractured_mods: Vec<String>,

    // Status flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrupted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<bool>,

    // Frame type (visual style: normal, magic, rare, unique, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_filename: Option<String>,

    // User notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
