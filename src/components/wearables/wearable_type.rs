use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
use enum_iterator::IntoEnumIterator;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::{
    equipment::{location_tag::LocationTag, Equipment},
    material::{BuiltWithMaterial, Material},
    object_descriptor::ObjectDescriptor,
    object_tag::{ObjectTag, TaggedObject},
};

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum WearableType {
    Breastplate,
    Mask,
    Cloak,
    Shirt,
    Trousers,
    Crown,
    Boots,
    Gloves,
    LoinCloth,
    PlateBoots,
    PlateGauntlets,
    PlateHelmet,
    Shackles,
    Vest,
}

impl WearableType {
    pub fn all() -> Vec<WearableType> {
        WearableType::into_enum_iter().collect()
    }

    pub fn is_multiple(&self) -> bool {
        match *self {
            WearableType::Breastplate => true,
            WearableType::Cloak => false,
            WearableType::Shirt => false,
            WearableType::PlateHelmet => false,
            WearableType::Shackles => false,
            WearableType::Mask => false,
            WearableType::Trousers => true,
            WearableType::Crown => false,
            WearableType::Boots => true,
            WearableType::Gloves => true,
            WearableType::LoinCloth => false,
            WearableType::PlateBoots => true,
            WearableType::PlateGauntlets => true,
            WearableType::Vest => false,
        }
    }

    pub fn unable_to_be_used_with(&self, other: &WearableType) -> bool {
        match *self {
            WearableType::Breastplate => other.is_upper_body(),
            WearableType::Boots => other.is_footwear(),
            WearableType::Cloak => other == &WearableType::Cloak,
            WearableType::Crown => other.is_headgear(),
            WearableType::Gloves => other.is_for_hands(),
            WearableType::LoinCloth => other.is_lower_body(),
            WearableType::Mask => other.is_headgear(),
            WearableType::PlateBoots => other.is_footwear(),
            WearableType::PlateGauntlets => other.is_footwear(),
            WearableType::PlateHelmet => other.is_headgear(),
            WearableType::Shackles => other.is_for_hands(),
            WearableType::Shirt => other.is_upper_body(),
            WearableType::Trousers => other.is_lower_body(),
            WearableType::Vest => other.is_upper_body(),
        }
    }

    pub fn is_lower_body(&self) -> bool {
        matches!(*self, WearableType::LoinCloth | WearableType::Trousers)
    }

    pub fn is_headgear(&self) -> bool {
        matches!(
            *self,
            WearableType::Crown | WearableType::Mask | WearableType::PlateHelmet
        )
    }

    pub fn is_upper_body(&self) -> bool {
        matches!(
            *self,
            WearableType::Breastplate | WearableType::Shirt | WearableType::Vest
        )
    }

    pub fn is_footwear(&self) -> bool {
        matches!(*self, WearableType::Boots | WearableType::PlateBoots)
    }

    pub fn is_for_hands(&self) -> bool {
        matches!(
            *self,
            WearableType::Shackles | WearableType::Boots | WearableType::PlateBoots
        )
    }

    pub fn necessary_descriptors(&self) -> Vec<ObjectDescriptor> {
        match *self {
            WearableType::Breastplate => Vec::new(),
            WearableType::Cloak => Vec::new(),
            WearableType::Shirt => Vec::new(),
            WearableType::PlateHelmet => Vec::new(),
            WearableType::Shackles => vec![ObjectDescriptor::SetOf],
            WearableType::Mask => Vec::new(),
            WearableType::Trousers => Vec::new(),
            WearableType::Crown => Vec::new(),
            WearableType::Boots => Vec::new(),
            WearableType::Gloves => Vec::new(),
            WearableType::LoinCloth => Vec::new(),
            WearableType::PlateBoots => Vec::new(),
            WearableType::PlateGauntlets => Vec::new(),
            WearableType::Vest => Vec::new(),
        }
    }
}

impl BuiltWithMaterial for WearableType {
    fn possible_materials(&self) -> Vec<Material> {
        match *self {
            WearableType::Breastplate => vec![Material::Iron, Material::Leather, Material::Steel],
            WearableType::Mask => vec![Material::Bone, Material::Iron],
            WearableType::Cloak => {
                vec![Material::Linen, Material::Hide, Material::Wool]
            }
            WearableType::Shirt => vec![
                Material::Wool,
                Material::Linen,
                Material::Cotton,
                Material::Silk,
            ],
            WearableType::Trousers => vec![
                Material::Hide,
                Material::Leather,
                Material::Wool,
                Material::Linen,
            ],
            WearableType::Crown => {
                vec![Material::Bone, Material::Gold, Material::Stone]
            }
            WearableType::Boots => vec![
                Material::Hide,
                Material::Iron,
                Material::Leather,
                Material::Steel,
            ],
            WearableType::Gloves => vec![Material::Hide, Material::Leather],
            WearableType::LoinCloth => vec![
                Material::Hide,
                Material::Wool,
                Material::Leather,
                Material::Silk,
                Material::Linen,
                Material::Cotton,
            ],
            WearableType::PlateBoots => vec![Material::Iron, Material::Steel],
            WearableType::PlateGauntlets => vec![Material::Iron, Material::Steel],
            WearableType::PlateHelmet => vec![Material::Iron, Material::Steel],
            WearableType::Shackles => vec![Material::Iron, Material::Leather, Material::Steel],
            WearableType::Vest => {
                vec![Material::Fur, Material::Hide, Material::Leather]
            }
        }
    }
}

impl TaggedObject for WearableType {
    fn tags(&self) -> Vec<ObjectTag> {
        match *self {
            WearableType::Breastplate => vec![ObjectTag::Armour],
            WearableType::Mask => vec![ObjectTag::Accessory],
            WearableType::Cloak => vec![ObjectTag::Clothing],
            WearableType::Shirt => vec![ObjectTag::Clothing],
            WearableType::Trousers => vec![ObjectTag::Clothing],
            WearableType::Crown => vec![ObjectTag::Accessory],
            WearableType::Boots => vec![ObjectTag::Armour],
            WearableType::Gloves => vec![ObjectTag::Clothing],
            WearableType::LoinCloth => vec![ObjectTag::Clothing],
            WearableType::PlateBoots => vec![ObjectTag::Armour],
            WearableType::PlateGauntlets => vec![ObjectTag::Armour],
            WearableType::PlateHelmet => vec![ObjectTag::Armour],
            WearableType::Shackles => vec![ObjectTag::Accessory, ObjectTag::Rope],
            WearableType::Vest => vec![ObjectTag::Clothing],
        }
    }
}

impl Equipment for WearableType {
    fn possible_location_tags(&self) -> Vec<LocationTag> {
        match *self {
            WearableType::Breastplate => vec![
                LocationTag::Equipped,
                LocationTag::Body,
                LocationTag::Packed,
            ],
            WearableType::Cloak => vec![
                LocationTag::Equipped,
                LocationTag::Shoulder,
                LocationTag::Packed,
            ],
            WearableType::Shirt => vec![
                LocationTag::Equipped,
                LocationTag::Body,
                LocationTag::Packed,
            ],
            WearableType::PlateHelmet => vec![
                LocationTag::Equipped,
                LocationTag::Head,
                LocationTag::Packed,
            ],
            WearableType::Shackles => vec![
                LocationTag::Equipped,
                LocationTag::Wrist,
                LocationTag::Packed,
            ],
            WearableType::Mask => vec![
                LocationTag::Equipped,
                LocationTag::Head,
                LocationTag::Packed,
            ],
            WearableType::Trousers => {
                vec![LocationTag::Equipped, LocationTag::Leg, LocationTag::Packed]
            }
            WearableType::Crown => vec![
                LocationTag::Equipped,
                LocationTag::Head,
                LocationTag::Packed,
            ],
            WearableType::Boots => vec![
                LocationTag::Equipped,
                LocationTag::Feet,
                LocationTag::Packed,
            ],
            WearableType::Gloves => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WearableType::LoinCloth => vec![
                LocationTag::Equipped,
                LocationTag::Waist,
                LocationTag::Packed,
            ],
            WearableType::PlateBoots => vec![
                LocationTag::Equipped,
                LocationTag::Feet,
                LocationTag::Packed,
            ],
            WearableType::PlateGauntlets => vec![
                LocationTag::Equipped,
                LocationTag::Hand,
                LocationTag::Packed,
            ],
            WearableType::Vest => vec![
                LocationTag::Equipped,
                LocationTag::Body,
                LocationTag::Packed,
            ],
        }
    }

    fn character_location_tags(&self) -> Vec<LocationTag> {
        match *self {
            WearableType::Breastplate => vec![LocationTag::Body],
            WearableType::Mask => vec![LocationTag::Head],
            WearableType::Cloak => vec![LocationTag::Shoulder],
            WearableType::Shirt => vec![LocationTag::Body],
            WearableType::Trousers => vec![LocationTag::Leg],
            WearableType::Crown => vec![LocationTag::Head],
            WearableType::Boots => vec![LocationTag::Feet],
            WearableType::Gloves => vec![LocationTag::Hand],
            WearableType::LoinCloth => vec![LocationTag::Waist],
            WearableType::PlateBoots => vec![LocationTag::Feet],
            WearableType::PlateGauntlets => vec![LocationTag::Hand],
            WearableType::PlateHelmet => vec![LocationTag::Head],
            WearableType::Shackles => vec![LocationTag::Ankle, LocationTag::Wrist],
            WearableType::Vest => vec![LocationTag::Body],
        }
    }
}

impl Display for WearableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WearableType::Breastplate => write!(f, "armour"),
            WearableType::Cloak => write!(f, "cloak"),
            WearableType::Shirt => write!(f, "shirt"),
            WearableType::PlateHelmet => write!(f, "plate helmet"),
            WearableType::Shackles => write!(f, "shackles"),
            WearableType::Mask => write!(f, "mask"),
            WearableType::Trousers => write!(f, "trousers"),
            WearableType::Crown => write!(f, "crown"),
            WearableType::Boots => write!(f, "boots"),
            WearableType::Gloves => write!(f, "gloves"),
            WearableType::LoinCloth => write!(f, "loin cloth"),
            WearableType::PlateBoots => write!(f, "plate boots"),
            WearableType::PlateGauntlets => write!(f, "plate gauntlets"),
            WearableType::Vest => write!(f, "vest"),
        }
    }
}
