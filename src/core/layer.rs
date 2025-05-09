use std::collections::HashMap;

use smallvec::SmallVec;

use crate::config::schema::{KeyAction, LayerDefinition, LayerModifierKind};

use super::{adapter::InputResult, shared::RawKeyCode};

#[derive(Debug, Clone)]
pub struct LayerItem {
    modifier: RawKeyCode,
    base_layer: Option<RawKeyCode>,
}

#[derive(Debug)]
pub struct LayerManager {
    layer_map: HashMap<RawKeyCode, LayerDefinition>,
    layer_stack: SmallVec<[LayerItem; 5]>,
    pending: SmallVec<[LayerItem; 5]>,
}

impl LayerManager {
    pub fn new(definitions: HashMap<String, LayerDefinition>) -> Self {
        Self {
            layer_map: definitions
                .into_values()
                .map(|value| (value.modifier.get_modifer().value(), value))
                .collect(),
            layer_stack: SmallVec::default(),
            pending: SmallVec::default(),
        }
    }

    pub fn map(&mut self, action: KeyAction) -> KeyAction {
        let KeyAction::KeyCode(code) = &action else {
            return action;
        };

        for layer in self.layer_stack.iter().rev() {
            let mapped_action = self
                .layer_map
                .get(&layer.modifier)
                .and_then(|layer| layer.keys.get(code));

            if let Some(action) = mapped_action {
                return action.clone();
            };
        }

        action
    }

    pub fn handle_press(&mut self, code: RawKeyCode) -> Option<InputResult> {
        if let Some(config) = self.layer_map.get(&code) {
            match config.modifier.get_modifer_kind() {
                LayerModifierKind::Toggle if self.is_layer_active(code) => self.pop_layer(code),
                _ if !self.is_layer_active(code) => self.push_layer(code),
                _ => {}
            };

            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_hold(&mut self, code: RawKeyCode) -> Option<InputResult> {
        if self.is_layer_active(code) {
            Some(InputResult::None)
        } else {
            None
        }
    }

    pub fn handle_release(&mut self, code: RawKeyCode) -> Option<InputResult> {
        if let Some(definition) = self.layer_map.get(&code) {
            if let LayerModifierKind::Momentary = definition.modifier.get_modifer_kind() {
                if let Some(dependent) = self.find_dependent_layer(code) {
                    self.pending.push(dependent);
                } else {
                    self.pop_layer(code);
                }
            }

            Some(InputResult::None)
        } else {
            if let Some(layer) = self.get_oneshoot_layer() {
                self.pop_layer(layer.modifier);
            }

            None
        }
    }

    fn is_layer_active(&self, modifier: RawKeyCode) -> bool {
        self.layer_stack
            .iter()
            .any(|layer| layer.modifier == modifier)
    }

    fn push_layer(&mut self, modifier: RawKeyCode) {
        let base_layer = self.layer_stack.last().map(|value| value.modifier);

        self.layer_stack.push(LayerItem {
            modifier,
            base_layer,
        });
    }

    fn pop_layer(&mut self, modifier: RawKeyCode) {
        if let Some(pending_layer) = self.find_pending_layer(modifier) {
            self.pending
                .retain(|layer| layer.modifier != pending_layer.modifier);

            if let Some(base_layer) = pending_layer.base_layer {
                self.pop_layer(base_layer);
            }
        }

        self.layer_stack.retain(|layer| layer.modifier != modifier);
    }

    fn find_pending_layer(&self, modifier: RawKeyCode) -> Option<LayerItem> {
        self.pending
            .iter()
            .find(|layer| layer.modifier == modifier)
            .cloned()
    }

    fn find_dependent_layer(&self, modifier: RawKeyCode) -> Option<LayerItem> {
        self.layer_stack
            .iter()
            .rev()
            .find(|layer| layer.base_layer.is_some_and(|base| base == modifier))
            .cloned()
    }

    fn get_oneshoot_layer(&self) -> Option<LayerItem> {
        self.layer_stack
            .iter()
            .rev()
            .find(|layer| self.is_oneshoot_layer(layer))
            .cloned()
    }

    fn is_oneshoot_layer(&self, layer: &LayerItem) -> bool {
        self.layer_map
            .get(&layer.modifier)
            .map(|layer| layer.modifier.get_modifer_kind())
            .is_some_and(|kind| matches!(kind, LayerModifierKind::Oneshoot))
    }
}
