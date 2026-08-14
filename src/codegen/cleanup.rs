use serde_json::{
    Map,
    Value,
};

const GRID_SIZE: i64 = 40;
const MARGIN: i64 = 8;
const SCRIPT_SPACING: i64 = GRID_SIZE * 2;

const HAT_PRIORITY: &[&str] = &[
    "event_whenflagclicked",
    "event_whenkeypressed",
    "event_whenthisspriteclicked",
    "event_whenbroadcastreceived",
    "event_whengreaterthan",
    "control_start_as_clone",
    "procedures_definition",
];
const C_SHAPED_OPCODES: &[&str] = &[
    "control_repeat",
    "control_repeat_until",
    "control_while",
    "control_for_each",
    "control_forever",
    "control_if",
    "control_if_else",
    "control_all_at_once",
];

pub(super) fn clean(project: &mut Value) {
    for target in project["targets"].as_array_mut().unwrap() {
        clean_target(target);
    }
}

fn clean_target(target: &mut Value) {
    let blocks = target["blocks"].as_object_mut().unwrap();
    let mut parents: Vec<_> = blocks
        .iter()
        .filter(|(_, block)| block["topLevel"].as_bool() == Some(true))
        .map(|(id, block)| (id.clone(), hat_priority(block)))
        .collect();
    parents.sort_by_key(|parent| parent.1);

    let mut y = 0;
    for (id, _) in parents {
        let height = height_stack(blocks, Some(&id))
            + (blocks[&id]["opcode"] == "procedures_definition") as i64 * MARGIN * 2;
        let block = blocks[&id].as_object_mut().unwrap();
        block.insert("x".into(), 0.into());
        block.insert("y".into(), y.into());
        y += height + SCRIPT_SPACING;
    }
}

fn hat_priority(block: &Value) -> usize {
    let opcode = block["opcode"].as_str().unwrap();
    HAT_PRIORITY
        .iter()
        .position(|it| *it == opcode)
        .unwrap_or(1000)
}

fn height_stack(blocks: &Map<String, Value>, id: Option<&str>) -> i64 {
    let Some(id) = id else { return 0 };
    let block = &blocks[id];
    let opcode = block["opcode"].as_str().unwrap();
    let inputs = block["inputs"].as_object();
    let mut height = GRID_SIZE + MARGIN + opcode.starts_with("pen_") as i64 * MARGIN;
    let mut input_height = 0;
    let mut nested_height = 0;

    if let Some(inputs) = inputs {
        for (name, input) in inputs {
            let kind = input[0].as_u64();
            if kind == Some(2) && matches!(name.as_str(), "SUBSTACK" | "SUBSTACK2") {
                nested_height += height_stack(blocks, input[1].as_str()).max(24) + 32;
            } else if matches!(kind, Some(2 | 3)) {
                input_height = input_height.max(height_reporter(blocks, &input[1]) + MARGIN);
            }
        }
    }
    if C_SHAPED_OPCODES.contains(&opcode)
        && inputs.is_none_or(|inputs| !inputs.contains_key("SUBSTACK"))
    {
        nested_height += 24 + 32;
    }
    if opcode == "control_if_else" && inputs.is_none_or(|inputs| !inputs.contains_key("SUBSTACK2"))
    {
        nested_height += 24 + 32;
    }

    height = height.max(input_height) + nested_height;
    height + height_stack(blocks, block["next"].as_str())
}

fn height_reporter(blocks: &Map<String, Value>, id: &Value) -> i64 {
    let Some(id) = id.as_str() else {
        return if id.is_null() { 0 } else { GRID_SIZE };
    };
    let block = &blocks[id];
    let mut height = GRID_SIZE;
    if let Some(inputs) = block["inputs"].as_object() {
        for input in inputs.values() {
            if matches!(input[0].as_u64(), Some(2 | 3)) && !input[1].is_null() {
                height = height.max(height_reporter(blocks, &input[1]) + MARGIN);
            }
        }
    }
    height
}
