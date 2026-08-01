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
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    pub name: String,
    /// (display_text, augmentation_type)
    pub values: Vec<(String, u32)>,
    #[serde(default)]
    pub display_mode: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub prop_type: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// Socket information for an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSocket {
    pub group: u32,
    /// PoE1 only: `S`, `D`, `I`, `G`, `A`, or `DV`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr: Option<String>,
    /// PoE1 only: `R`, `G`, `B`, `W`, `A`, or `DV`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_colour: Option<String>,
    /// PoE2 only: `gem`, `jewel`, or `rune`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub socket_type: Option<String>,
    /// PoE2 only: `emerald`, `sapphire`, `ruby`, `rune`, `soulcore`, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}

/// A PoE2 item from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    // Identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    #[serde(default)]
    pub rarity: Option<String>,
    pub identified: bool,
    pub verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,

    // Dimensions
    pub w: u32,
    pub h: u32,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<bool>,

    // Levels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ilvl: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unidentified_tier: Option<i32>,

    // Stack info (for currency, fragments, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack_size_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_tier_text: Option<String>,

    // Properties and requirements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notable_properties: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon_requirements: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_gem_requirements: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tamed_beast_properties: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_level_requirements: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_skills: Option<Vec<ItemProperty>>,

    // Sockets
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem_sockets: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sockets: Option<Vec<ItemSocket>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socketed_items: Option<Vec<Item>>,

    // Mods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implicit_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_mods: Option<Vec<ModInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crafted_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enchant_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rune_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fractured_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonded_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutated_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utility_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cosmetic_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub veiled_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desecrated_mods: Option<Vec<String>>,

    // Descriptions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descr_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sec_descr_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavour_text: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavour_text_note: Option<String>,

    // Status flags
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrupted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_corrupted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplicated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_to_character: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_to_account: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmodifiable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmodifiable_except_chaos: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanctified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub veiled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desecrated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruthless: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_relic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreseeing: Option<bool>,

    // Frame and visual
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub art_filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foil_variation: Option<i32>,

    // Gem-specific (PoE2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem_tabs: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gem_skill: Option<String>,

    // Talisman
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talisman_tier: Option<i32>,

    // Incubated item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incubated_item: Option<IncubatedItem>,

    // Rewards
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<Reward>>,

    // Hybrid (for Vaal gems)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<HybridInfo>,

    // Extended (Public Stash API only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended: Option<ExtendedInfo>,

    // Positioning (for stash tabs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socket: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socketed_icon: Option<String>,

    // User notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_note: Option<String>,
}

/// Incubated item information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncubatedItem {
    pub name: String,
    pub level: u32,
    pub progress: u32,
    pub total: u32,
}

/// A mod on an item (from the Trade API explicitMods format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    /// Human-readable description with optional [tags]
    pub description: String,
    /// Stat hash identifier (e.g., "stat.explicit.stat_3291658075")
    pub hash: String,
    /// Mod roll values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mods: Option<Vec<ModRoll>>,
}

/// A single mod roll with magnitude ranges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModRoll {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnitudes: Option<Vec<Magnitude>>,
}

/// A min/max magnitude for a mod value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Magnitude {
    pub min: String,
    pub max: String,
}

/// Extended item information (Public Stash API / Trade API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedInfo {
    /// Detailed mod info by mod type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mods: Option<serde_json::Value>,
    /// Stat hashes by mod type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashes: Option<serde_json::Value>,
}

/// Reward information (e.g., from league mechanics)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub label: String,
    pub rewards: std::collections::HashMap<String, i32>,
}

/// Hybrid item information (e.g., Vaal gems)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_vaal_gem: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ItemProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sec_descr_text: Option<String>,
}

#[cfg(test)]
mod tests;
