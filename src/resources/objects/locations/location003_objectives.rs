use std::ops::Not;
use crate::model::def::{GameResource, ProgressFlag, ProgressPredicate, ProgressRule};

pub fn objectives() -> Vec<ProgressRule> {
    vec![
        ProgressRule {
            objective: Some("- explore islands for the Ancient data pieces"),
            journal_entry: Some(vec![
                "You've found an excellent piece of Ancient wisdom!",
                "It perfectly fits the slot in one of the Ancient Devices",
                "known as FDD. After examining monstrous amount of data on",
                "this piece, you've found out that some very threatening",
                "technology is written on the two so-called Compact Disks",
                "hidden in this archipelago."
            ]),
            display_condition: ProgressPredicate(|game| !game.progression.flags.contains(&ProgressFlag::FloppiesFound)),
            complete_condition: ProgressPredicate(|game| game.player.resources.get(&GameResource::A).copied().unwrap_or(0) >= 1),
            output_flags: vec![ProgressFlag::FloppiesFound],
        },
        ProgressRule {
            objective: Some("- find 2 Ancient Laser Disks!"),
            journal_entry: Some(vec![
                "Finally, I've found these Laser Disks. It says:",
                "\"ti yalp annog er'ew dna emag eht wonk eW",
                "no gniog neeb s'tahw wonk htob ew ,edisnI ti",
                "yas ot yhs oot er'uoy tub ,gnihca neeb s'traeh",
                "ruoY gnol os rof rehto hcae nwonk ev'eW\"",
                "Elders says that that's encrypted prophecy and",
                "to decrypt we need to get three parts of key on",
                "Ancient SD Cards!"
            ]),
            display_condition: ProgressPredicate(|game|
                game.progression.flags.contains(&ProgressFlag::FloppiesFound)
                    && !game.progression.flags.contains(&ProgressFlag::DisksFound)
            ),
            complete_condition: ProgressPredicate(|game| game.player.resources.get(&GameResource::B).copied().unwrap_or(0) >= 2),
            output_flags: vec![ProgressFlag::DisksFound],
        },
        ProgressRule {
            objective: Some("- find 3 Ancient SD Card!!!"),
            journal_entry: Some(vec![
                "That's all content for now. Enjoy your Victory :)"
            ]),
            display_condition: ProgressPredicate(|game|
                game.progression.flags.contains(&ProgressFlag::DisksFound)
                    && !game.progression.flags.contains(&ProgressFlag::SdCardsFound)
            ),
            complete_condition: ProgressPredicate(|game| game.player.resources.get(&GameResource::C).copied().unwrap_or(0) >= 3),
            output_flags: vec![ProgressFlag::SdCardsFound],
        },
        ProgressRule {
            objective: Some("- Enjoy your Victory :)"),
            journal_entry: Some(vec![]),
            display_condition: ProgressPredicate(|game|
                game.progression.flags.contains(&ProgressFlag::FloppiesFound)
                    && game.progression.flags.contains(&ProgressFlag::DisksFound)
                    && game.progression.flags.contains(&ProgressFlag::SdCardsFound)
            ),
            complete_condition: ProgressPredicate(|game| false),
            output_flags: vec![],
        },
    ]
}