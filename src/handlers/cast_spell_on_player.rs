use std::error::Error;

use crate::{
    actions::cast_spell_on_player::CastSpellOnPlayer,
    components::{
        damage::{Attack, Defense},
        player::PlayerCharacter,
        spells::spell_name::SpellName,
    },
    errors::spell_not_found_error::SpellNotFoundError,
    events::{
        event::Event, player_gains_resurrection_aura::PlayerGainsResurrectionAura,
        player_gains_retribution_aura::PlayerGainsRetributionAura,
        player_gains_shield_aura::PlayerGainsShieldAura, player_healed::PlayerHealed,
        player_hit::PlayerHit,
    },
    utils::ids::parse_id,
};

pub fn handle(
    cast_spell_on_player: &CastSpellOnPlayer,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let spell_id = parse_id(&cast_spell_on_player.spell_id)?;
    let learned_spell = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => return Err(Box::new(SpellNotFoundError(spell_id.to_string()))),
    };

    let mut events: Vec<Event> = Vec::new();
    match learned_spell.spell.name {
        SpellName::ElectricBlast | SpellName::RagingFireball => {
            let damage = learned_spell.spell.damage();
            events.push(Event::PlayerHit(PlayerHit {
                attacker_id: player.identifier.id,
                damage,
            }));
        }
        SpellName::Heal | SpellName::QuickHeal => {
            let damage_healed = learned_spell.spell.damage();
            events.push(Event::PlayerHealed(PlayerHealed { damage_healed }));
        }
        SpellName::Phoenix => events.push(Event::PlayerGainsResurrectionAura(
            PlayerGainsResurrectionAura,
        )),
        SpellName::Retribution => {
            let attack = learned_spell.spell.attack.clone().unwrap_or(Attack {
                num_rolls: 2,
                modifier: 0,
            });
            events.push(Event::PlayerGainsRetributionAura(
                PlayerGainsRetributionAura { attack },
            ));
        }
        SpellName::TinyShield => {
            let defense = learned_spell.spell.defense.clone().unwrap_or(Defense {
                damage_resistance: 6,
            });
            events.push(Event::PlayerGainsShieldAura(PlayerGainsShieldAura {
                defense,
            }));
        }
    }

    Ok(events)
}