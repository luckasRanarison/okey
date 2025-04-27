use std::collections::HashMap;

use crate::config::schema::{KeyAction, LayerDefinition, LayerModifierKind};

use super::adapter::InputResult;

#[derive(Debug)]
pub struct LayerManager {
    layer_map: HashMap<u16, LayerDefinition>,
    layer_stack: Vec<u16>,
}

impl LayerManager {
    pub fn new(definitions: HashMap<String, LayerDefinition>) -> Self {
        Self {
            layer_map: definitions
                .into_values()
                .map(|value| (value.modifier.get_modifer().value(), value))
                .collect(),
            layer_stack: Vec::with_capacity(5),
        }
    }

    pub fn map(&mut self, action: KeyAction) -> KeyAction {
        let KeyAction::KeyCode(code) = &action else {
            return action;
        };

        for layer in self.layer_stack.iter().rev() {
            let mapped_action = self
                .layer_map
                .get(layer)
                .and_then(|layer| layer.keys.get(code));

            if let Some(action) = mapped_action {
                return action.clone();
            };
        }

        action
    }

    pub fn handle_press(&mut self, code: u16) -> Option<InputResult> {
        if let Some(config) = self.layer_map.get(&code) {
            match config.modifier.get_modifer_kind() {
                LayerModifierKind::Toggle if self.layer_stack.contains(&code) => {
                    self.layer_stack.retain(|layer| *layer != code)
                }
                _ if !self.layer_stack.contains(&code) => self.layer_stack.push(code),
                _ => {}
            };

            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: u16) -> Option<InputResult> {
        if self.layer_stack.contains(&code) {
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_release(&mut self, code: u16) -> Option<InputResult> {
        if let Some(definition) = self.layer_map.get(&code) {
            if let LayerModifierKind::Momentary = definition.modifier.get_modifer_kind() {
                self.layer_stack.retain(|layer_code| *layer_code != code)
            }

            Some(InputResult::None)
        } else {
            None
        }
    }
}
