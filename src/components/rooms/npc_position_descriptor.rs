use std::fmt::Display;

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub enum NpcPositionDescriptor {
    AreGlaringAtYou,
    AreGlaringAtYouFromNearby,
    IsGlaringAtYou,
    IsGlaringAtYouFromNearby,
    AreInTheCorner,
    InTheCornerAre,
    InCornerStands,
    AreLoiteringAbout,
    AreStandingAround,
    IsStandingAround,
    LeansAgainstTheTable,
    AreLeaningAgainstTheTable,
    IsLeaningAgainstTheTable,
    StandsOnTheTable,
    AreStandingOnTheTable,
    IsStandingOnTheTable,
    IsSittingInAChair,
    AreSittingInChairs,
    IsStandingInABarrel,
    IsSleepingInTheBed,
    IsSleepingInACot,
    IsSleepingInSleepingRoll,
    AreLeaningOnACrate,
    IsLeaningOnACrate,
    AreLookingAtTheWeaponRack,
    IsLookingAtTheWeaponRack,
    IsCrouchedOverChest,
    IsRummagingThroughAChest,
    AreCrouchedInTheCenterOfRoom,
    IsCrouchedInTheCenterOfRoom,
    IsSittingAndDozingInCenterOfRoom,
    SittingInAChairIs,
}

impl NpcPositionDescriptor {
    fn descriptor_position(&self) -> DescriptorPosition {
        match *self {
            NpcPositionDescriptor::AreGlaringAtYou => DescriptorPosition::Post,
            NpcPositionDescriptor::AreStandingAround => DescriptorPosition::Post,
            NpcPositionDescriptor::AreGlaringAtYouFromNearby => DescriptorPosition::Post,
            NpcPositionDescriptor::AreLoiteringAbout => DescriptorPosition::Post,
            NpcPositionDescriptor::AreInTheCorner => DescriptorPosition::Post,
            NpcPositionDescriptor::InTheCornerAre => DescriptorPosition::Pre,
            NpcPositionDescriptor::InCornerStands => DescriptorPosition::Pre,
            NpcPositionDescriptor::IsGlaringAtYou => DescriptorPosition::Post,
            NpcPositionDescriptor::IsGlaringAtYouFromNearby => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingAround => DescriptorPosition::Post,
            NpcPositionDescriptor::LeansAgainstTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::AreLeaningAgainstTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLeaningAgainstTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::StandsOnTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::AreStandingOnTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingOnTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSittingInAChair => DescriptorPosition::Post,
            NpcPositionDescriptor::AreSittingInChairs => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingInABarrel => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInTheBed => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInACot => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInSleepingRoll => DescriptorPosition::Post,
            NpcPositionDescriptor::AreLeaningOnACrate => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLeaningOnACrate => DescriptorPosition::Post,
            NpcPositionDescriptor::AreLookingAtTheWeaponRack => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLookingAtTheWeaponRack => DescriptorPosition::Post,
            NpcPositionDescriptor::IsCrouchedOverChest => DescriptorPosition::Post,
            NpcPositionDescriptor::AreCrouchedInTheCenterOfRoom => DescriptorPosition::Post,
            NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSittingAndDozingInCenterOfRoom => DescriptorPosition::Post,
            NpcPositionDescriptor::SittingInAChairIs => DescriptorPosition::Pre,
            NpcPositionDescriptor::IsRummagingThroughAChest => DescriptorPosition::Post,
        }
    }

    pub fn unable_to_be_used_with(&self, other: NpcPositionDescriptor) -> bool {
        match *self {
            NpcPositionDescriptor::AreGlaringAtYou => other.is_post(),
            NpcPositionDescriptor::AreGlaringAtYouFromNearby => other.is_post(),
            NpcPositionDescriptor::AreInTheCorner => other.is_pre(),
            NpcPositionDescriptor::InCornerStands => other.is_pre(),
            NpcPositionDescriptor::AreLoiteringAbout => other.is_post(),
            NpcPositionDescriptor::AreStandingAround => other.is_post(),
            NpcPositionDescriptor::IsGlaringAtYou => other.is_post(),
            NpcPositionDescriptor::IsGlaringAtYouFromNearby => false,
            NpcPositionDescriptor::IsStandingAround => false,
            NpcPositionDescriptor::LeansAgainstTheTable => false,
            NpcPositionDescriptor::AreLeaningAgainstTheTable => false,
            NpcPositionDescriptor::IsLeaningAgainstTheTable => false,
            NpcPositionDescriptor::StandsOnTheTable => false,
            NpcPositionDescriptor::AreStandingOnTheTable => false,
            NpcPositionDescriptor::IsStandingOnTheTable => false,
            NpcPositionDescriptor::IsSittingInAChair => false,
            NpcPositionDescriptor::AreSittingInChairs => false,
            NpcPositionDescriptor::IsStandingInABarrel => false,
            NpcPositionDescriptor::IsSleepingInTheBed => false,
            NpcPositionDescriptor::IsSleepingInACot => false,
            NpcPositionDescriptor::IsSleepingInSleepingRoll => false,
            NpcPositionDescriptor::AreLeaningOnACrate => false,
            NpcPositionDescriptor::IsLeaningOnACrate => false,
            NpcPositionDescriptor::AreLookingAtTheWeaponRack => false,
            NpcPositionDescriptor::IsLookingAtTheWeaponRack => false,
            NpcPositionDescriptor::IsCrouchedOverChest => false,
            NpcPositionDescriptor::AreCrouchedInTheCenterOfRoom => false,
            NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom => false,
            NpcPositionDescriptor::IsSittingAndDozingInCenterOfRoom => false,
            NpcPositionDescriptor::SittingInAChairIs => false,
            NpcPositionDescriptor::IsRummagingThroughAChest => false,
            NpcPositionDescriptor::InTheCornerAre => false,
        }
    }

    pub fn is_pre(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Pre
    }

    pub fn is_post(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Post
    }
}

impl Display for NpcPositionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            NpcPositionDescriptor::AreGlaringAtYouFromNearby => "are glaring at you from nearby",
            NpcPositionDescriptor::AreLoiteringAbout => "are loitering about",
            NpcPositionDescriptor::AreStandingAround => "are standing around",
            NpcPositionDescriptor::AreInTheCorner => "are in the corner",
            NpcPositionDescriptor::InCornerStands => "in the corner stands",
            NpcPositionDescriptor::AreGlaringAtYou => "are glaring at you",
            NpcPositionDescriptor::IsGlaringAtYou => "is glaring at you",
            NpcPositionDescriptor::IsGlaringAtYouFromNearby => "is glaring at you from nearby",
            NpcPositionDescriptor::IsStandingAround => "is standing around",
            NpcPositionDescriptor::LeansAgainstTheTable => "leans against the table",
            NpcPositionDescriptor::AreLeaningAgainstTheTable => "are leaning against the table",
            NpcPositionDescriptor::IsLeaningAgainstTheTable => "is leaning against the table",
            NpcPositionDescriptor::StandsOnTheTable => "stands on the table",
            NpcPositionDescriptor::AreStandingOnTheTable => "are standing on the table",
            NpcPositionDescriptor::IsStandingOnTheTable => "is standing on the table",
            NpcPositionDescriptor::IsSittingInAChair => "is sitting in a chair",
            NpcPositionDescriptor::AreSittingInChairs => "are sitting in chairs",
            NpcPositionDescriptor::IsStandingInABarrel => "is standing in a barrel",
            NpcPositionDescriptor::IsSleepingInTheBed => "is sleeping in the bed",
            NpcPositionDescriptor::IsSleepingInACot => "is sleeping in a cot",
            NpcPositionDescriptor::IsSleepingInSleepingRoll => "is sleeping in a sleeping roll",
            NpcPositionDescriptor::AreLeaningOnACrate => "are leaning on a crate",
            NpcPositionDescriptor::IsLeaningOnACrate => "is leaning on a crate",
            NpcPositionDescriptor::AreLookingAtTheWeaponRack => "are looking at the weapon rack",
            NpcPositionDescriptor::IsLookingAtTheWeaponRack => "is looking at the weapon rack",
            NpcPositionDescriptor::IsCrouchedOverChest => "is crouched over a chest",
            NpcPositionDescriptor::AreCrouchedInTheCenterOfRoom => {
                "are crouched in the center of the room"
            }
            NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom => {
                "is crouched in the center of the room"
            }
            NpcPositionDescriptor::IsSittingAndDozingInCenterOfRoom => {
                "is sitting and dozing in center of the room"
            }
            NpcPositionDescriptor::SittingInAChairIs => "sitting in a chair is",
            NpcPositionDescriptor::IsRummagingThroughAChest => "is rummaging through a chest",
            NpcPositionDescriptor::InTheCornerAre => "in the corner are",
        };

        write!(f, "{}", text)
    }
}