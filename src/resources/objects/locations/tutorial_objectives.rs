use std::ops::Not;
use crate::model::def::{GameResource, ProgressFlag, ProgressPredicate, ProgressRule};

pub fn objectives() -> Vec<ProgressRule> {
    vec![
        ProgressRule {
            objective: None,
            journal_entry: Some(vec![
                "Hey stalker, elders said this archipelago is full of Ancient",
                "stuff. They said it's digital data and can be opened by",
                "putting it to the slots of devices we've found last raid.",
                "Tear them up!",
            ]),
            display_condition: ProgressPredicate(|game| game.progression.flags.contains(&ProgressFlag::BriefingShown).not()),
            complete_condition: ProgressPredicate(|game| true),
            output_flags: vec![ProgressFlag::BriefingShown]
        },
        ProgressRule {
            objective: None,
            journal_entry: Some(vec![
                "Hey stalker, we barely assembled you after that crash.",
                "Good for you, you we've repaired all your gear and you ",
                "haven't lost your memory",
            ]),
            display_condition: ProgressPredicate(|game| game.progression.flags.contains(&ProgressFlag::DeathTutorialShown).not()),
            complete_condition: ProgressPredicate(|game| game.player.death_count > 0),
            output_flags: vec![ProgressFlag::DeathTutorialShown]
        },

        ProgressRule {
            objective: None,
            journal_entry: Some(vec![
                "You've reached a new level of experience! Each level-up",
                "gives you one skill point. Skill points are accumulated",
                "and can be spent on improvements.",
                "F2 brings you to the Improvements shop.",
            ]),
            display_condition: ProgressPredicate(|game| game.progression.flags.contains(&ProgressFlag::LevelUpTutorialShown).not()),
            complete_condition: ProgressPredicate(|game| game.player.rpg.level > 1),
            output_flags: vec![ProgressFlag::LevelUpTutorialShown]
        },

        ProgressRule {
            objective: None,
            journal_entry: Some(vec![
                "You've got new weapon/utility equipment unit.",
                "We've already automatically mounted it to the vacant",
                "equipment slot, but feel free to customize it.",
                "F3 brings you to the equipment customization shop.",
            ]),
            display_condition: ProgressPredicate(|game| game.progression.flags.contains(&ProgressFlag::EquipmentTutorialShown).not()),
            complete_condition: ProgressPredicate(|game| game.player.equipment.len() > 1),
            output_flags: vec![ProgressFlag::EquipmentTutorialShown]
        },
    ]
}