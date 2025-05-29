use crate::model::state::Gids;

impl Gids {
    pub fn next_gid(&mut self) -> i64 {
        let m = self.prev_gid + 1;
        self.prev_gid = m;
        m
    }
}