use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::fixtures::fixture::Fixture;

use super::{
    fixture_position_descriptor::FixturePositionDescriptor, group_descriptor::GroupDescriptor,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixturePosition {
    #[cfg_attr(feature = "serialization", serde(default))]
    pub group_descriptor: Option<GroupDescriptor>,
    pub fixtures: Vec<Fixture>,
    pub position_descriptors: Vec<FixturePositionDescriptor>,
}

impl Display for FixturePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();

        for descriptor in self.position_descriptors.iter().filter(|d| d.is_pre()) {
            parts.push(format!("{}", descriptor));
        }

        if let Some(group_descriptor) = &self.group_descriptor {
            parts.push(format!("{}", group_descriptor));
        }

        parts.push(self.fixtures_description());

        for descriptor in self.position_descriptors.iter().filter(|d| d.is_post()) {
            parts.push(format!("{}", descriptor));
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl FixturePosition {
    pub fn fixtures_description(&self) -> String {
        let counts = crate::utils::frequencies::sorted_frequencies(
            self.fixtures.iter().map(|f| f.fixture_type.clone()),
        );

        let mut parts: Vec<String> = Vec::new();

        for (fixture, count) in counts {
            parts.push(fixture.describe_count(count));
        }

        parts.join(" and ")
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        fixtures::{fixture::Fixture, fixture_type::FixtureType},
        rooms::{
            fixture_position_descriptor::FixturePositionDescriptor,
            group_descriptor::GroupDescriptor,
        },
        size::Size,
    };

    use super::FixturePosition;

    #[test]
    fn display() {
        let table = Fixture {
            fixture_type: FixtureType::Table,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
        };
        let chair = Fixture {
            fixture_type: FixtureType::Chair,
            material: None,
            size: Size::Average,
            descriptors: Vec::new(),
        };
        let fixture_position = FixturePosition {
            group_descriptor: Some(GroupDescriptor::A),
            fixtures: vec![table, chair.clone(), chair],
            position_descriptors: vec![FixturePositionDescriptor::IsInTheCorner],
        };

        assert_eq!(
            "a table and chairs is in the corner",
            format!("{}", &fixture_position)
        )
    }
}
