use crate::rules::traits::{Rule, RuleAction};
use crate::rules::conditions::Condition;
use crate::types::cart::Cart;
use crate::core::errors::EngineResult;

/// ============================================================================
/// 🏗️ Rule Builder (රීති නිර්මාණකරු)
/// ============================================================================
/// ඉතා පහසුවෙන් රීති සෑදීම සඳහා මෙය භාවිතා කරයි (Fluent API).

pub struct SimpleRule {
    name: String,
    condition: Condition,
    action: Box<dyn Fn(&Cart) -> EngineResult<Vec<RuleAction>>>,
    priority: i32,
}

impl Rule for SimpleRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_apply(&self, cart: &Cart) -> bool {
        self.condition.evaluate(cart)
    }

    fn apply(&self, cart: &Cart) -> EngineResult<Vec<RuleAction>> {
        (self.action)(cart)
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}

pub struct RuleBuilder {
    name: String,
    condition: Option<Condition>,
    priority: i32,
}

impl RuleBuilder {
    pub fn new(name: &str) -> Self {
        RuleBuilder {
            name: name.to_string(),
            condition: None,
            priority: 0,
        }
    }

    pub fn when(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn then<F>(self, action: F) -> Box<dyn Rule>
    where
        F: Fn(&Cart) -> EngineResult<Vec<RuleAction>> + 'static,
    {
        Box::new(SimpleRule {
            name: self.name,
            condition: self.condition.expect("Condition must be set"),
            action: Box::new(action),
            priority: self.priority,
        })
    }
}
