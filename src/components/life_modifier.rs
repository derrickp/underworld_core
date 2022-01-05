use crate::describable::Describable;

#[derive(Clone, Debug)]
pub enum LifeModifier {
    Skeleton,
    Vampire,
    Zombie,
}

impl Describable for LifeModifier {
    fn describe(&self) -> String {
        match *self {
            Self::Skeleton => "skeleton".to_string(),
            Self::Vampire => "vampire".to_string(),
            Self::Zombie => "zombie".to_string(),
        }
    }
}

impl LifeModifier {
    pub fn as_adjective(&self) -> String {
        match *self {
            Self::Skeleton => "skeletal".to_string(),
            Self::Vampire => "vampiric".to_string(),
            Self::Zombie => "zombified".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{describable::Describable, components::life_modifier::LifeModifier};

    #[test]
    fn describe() {
        assert_eq!("zombie", LifeModifier::Zombie.describe());
        assert_eq!("vampire", LifeModifier::Vampire.describe());
        assert_eq!("skeleton", LifeModifier::Skeleton.describe());
    }

    #[test]
    fn as_adjective() {
        assert_eq!("zombified", LifeModifier::Zombie.as_adjective());
        assert_eq!("vampiric", LifeModifier::Vampire.as_adjective());
        assert_eq!("skeletal", LifeModifier::Skeleton.as_adjective());
    }
}