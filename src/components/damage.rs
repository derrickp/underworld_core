#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
use rand::prelude::ThreadRng;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Attack {
    pub num_rolls: usize,
    pub modifier: i32,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub effects: Vec<AttackEffect>,
}

#[derive(Clone, Debug, EnumIter, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum AttackEffect {
    Acidic,
    Crushing,
    Sharp,
    Toxic,
}

impl Attack {
    pub fn attack_roll(&self, rng: &mut ThreadRng) -> i32 {
        let roll = crate::utils::rolls::roll_d6(rng, self.num_rolls, self.modifier);
        if self
            .effects
            .iter()
            .any(|effect| matches!(*effect, AttackEffect::Crushing))
        {
            roll + (roll / 2)
        } else {
            roll
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Defense {
    pub damage_resistance: i32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "openapi", derive(Object))]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
