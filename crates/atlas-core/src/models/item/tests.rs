use crate::models::item::Item;

fn load_trade_item(json: &str) -> Option<Item> {
    let data: serde_json::Value = serde_json::from_str(json).ok()?;
    let item_value = &data["result"][0]["item"];
    serde_json::from_value(item_value.clone()).ok()
}

#[test]
fn deserialize_basic_fields() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    assert_eq!(item.realm.as_deref(), Some("poe2"));
    assert_eq!(item.name, "Polcirkeln");
    assert_eq!(item.type_line, "Sapphire Ring");
    assert_eq!(item.base_type, "Sapphire Ring");
    assert_eq!(item.rarity.as_deref(), Some("Unique"));
    assert_eq!(item.ilvl, Some(80));
    assert!(item.identified);
    assert!(item.verified);
    assert_eq!(item.corrupted, Some(true));
    assert_eq!(item.w, 1);
    assert_eq!(item.h, 1);
    assert!(item.icon.contains("Polcirkeln.png"));
}

#[test]
fn deserialize_string_mods() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    let Some(enchant) = &item.enchant_mods else {
        panic!("enchant_mods missing");
    };
    assert_eq!(enchant.len(), 1);
    assert!(enchant[0].contains("Dexterity"));

    let Some(implicit) = &item.implicit_mods else {
        panic!("implicit_mods missing");
    };
    assert_eq!(implicit.len(), 1);
    assert!(implicit[0].contains("Cold Resistance"));
}

#[test]
fn deserialize_explicit_mods_with_rolls() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    let Some(mods) = &item.explicit_mods else {
        panic!("explicit_mods missing");
    };
    assert_eq!(mods.len(), 4);

    let first = &mods[0];
    assert!(first.description.contains("Cold"));
    assert!(first.hash.starts_with("stat.explicit."));

    let Some(rolls) = &first.mods else {
        panic!("mod rolls missing");
    };
    assert_eq!(rolls.len(), 1);

    let Some(magnitudes) = &rolls[0].magnitudes else {
        panic!("magnitudes missing");
    };
    assert_eq!(magnitudes.len(), 1);
    assert_eq!(magnitudes[0].min, "20");
    assert_eq!(magnitudes[0].max, "30");
}

#[test]
fn deserialize_properties_and_requirements() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    let Some(props) = &item.properties else {
        panic!("properties missing");
    };
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].name, "Ring");
    assert_eq!(props[0].prop_type, Some(109));

    let Some(reqs) = &item.requirements else {
        panic!("requirements missing");
    };
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].name, "Level");
}

#[test]
fn deserialize_flavour_text() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    let Some(text) = &item.flavour_text else {
        panic!("flavour_text missing");
    };
    assert!(!text.is_empty());
    assert!(text[0].contains("north"));
}

#[test]
fn deserialize_extended_info() {
    let Some(item) = load_trade_item(include_str!("../test_item.json")) else {
        panic!("failed to deserialize test_item.json");
    };

    let Some(ext) = &item.extended else {
        panic!("extended missing");
    };
    assert!(ext.mods.is_some());
    assert!(ext.hashes.is_some());
}

#[test]
fn deserialize_sockets_and_socketed_items() {
    let json = r#"{
        "result": [{
            "item": {
                "name": "Body Armour",
                "typeLine": "Body Armour",
                "baseType": "Body Armour",
                "identified": true,
                "verified": true,
                "w": 2, "h": 3,
                "icon": "https://example.com/icon.png",
                "sockets": [
                    {"group": 0, "type": "rune"},
                    {"group": 1, "type": "rune"}
                ],
                "socketedItems": [{
                    "name": "Greater Storm Rune",
                    "typeLine": "Rune",
                    "baseType": "Rune",
                    "identified": true,
                    "verified": true,
                    "w": 1, "h": 1,
                    "icon": "https://example.com/rune.png"
                }],
                "properties": [],
                "requirements": []
            }
        }]
    }"#;

    let Some(item) = load_trade_item(json) else {
        panic!("failed to deserialize socket item");
    };

    let Some(sockets) = &item.sockets else {
        panic!("sockets missing");
    };
    assert_eq!(sockets.len(), 2);
    assert_eq!(sockets[0].group, 0);
    assert_eq!(sockets[0].socket_type.as_deref(), Some("rune"));

    let Some(socketed) = &item.socketed_items else {
        panic!("socketed_items missing");
    };
    assert_eq!(socketed.len(), 1);
    assert_eq!(socketed[0].name, "Greater Storm Rune");
}

#[test]
fn deserialize_rune_mods_and_mutated_flag() {
    let json = r#"{
        "result": [{
            "item": {
                "name": "Test Gloves",
                "typeLine": "Sectioned Bracers",
                "baseType": "Sectioned Bracers",
                "rarity": "Unique",
                "ilvl": 69,
                "identified": true,
                "verified": true,
                "w": 2, "h": 2,
                "icon": "https://example.com/icon.png",
                "mutated": true,
                "runeMods": ["+40 to maximum Mana", "+20 to maximum Life"],
                "properties": [],
                "requirements": []
            }
        }]
    }"#;

    let Some(item) = load_trade_item(json) else {
        panic!("failed to deserialize rune mod item");
    };

    assert_eq!(item.name, "Test Gloves");
    assert_eq!(item.mutated, Some(true));

    let Some(rune_mods) = &item.rune_mods else {
        panic!("rune_mods missing");
    };
    assert_eq!(rune_mods.len(), 2);
    assert!(rune_mods[0].contains("Mana"));
}
