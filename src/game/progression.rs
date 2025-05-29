use crate::common::pool::Pool;
use crate::GameState;
use crate::model::def::GameProgressCtx;
use crate::model::state::{JournalStatePage, JournalWindow, LootId, LootState, UiWindow};

pub fn update(state: &mut GameState) {
    for rule in state.location.progression.iter() {
        let ctx = GameProgressCtx { player: &state.player, progression: &state.progression, loot: &state.loot };
        if (rule.display_condition.0)(&ctx) {
            if (rule.complete_condition.0)(&ctx) {
                for flag in &rule.output_flags {
                    state.progression.flags.insert(*flag);
                }
                if let Some(entry) = &rule.journal_entry {
                    state.journal.push(JournalStatePage {
                        lines: entry.clone()
                    });
                    state.player.windows.pop_back();
                    state.player.windows.push_back(UiWindow::Journal(JournalWindow { page: state.journal.len() - 1 }));
                }
            }
        }
    }
}