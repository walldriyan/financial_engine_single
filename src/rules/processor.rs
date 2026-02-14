use crate::types::cart::Cart;
use crate::core::errors::EngineResult;
use crate::rules::traits::{Rule, RuleAction};

/// ============================================================================
/// ⚙️ Rule Processor (රීති ක්‍රියාත්මක කරන්නා)
/// ============================================================================
/// සියලුම රීති කළමනාකරණය කරන්නේ මොහුයි.
/// Priority අනුව රීති පෙළගස්වා එකින් එක ක්‍රියාත්මක කරයි.

pub struct RuleProcessor {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleProcessor {
    pub fn new() -> Self {
        RuleProcessor {
            rules: Vec::new(),
        }
    }

    /// 📥 රීතියක් එකතු කරන්න (Register Rule)
    pub fn register_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
        // Sort by priority (descending)
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// 🚀 සියලු රීති ක්‍රියාත්මක කරන්න (Process All)
    pub fn process(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        let mut actions = Vec::new();

        for rule in &self.rules {
            if rule.can_apply(cart) {
                // Apply the rule safely
                match rule.apply(cart) {
                    Ok(mut rule_actions) => {
                        actions.append(&mut rule_actions);
                    },
                    Err(e) => {
                        // Log error but maybe don't stop everything?
                        // For now we return error as per centralized policy
                        return Err(e);
                    }
                }
            }
        }

        Ok(actions)
    }
}
