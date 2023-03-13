pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player{health:100, mana: Some(100), level : self.level})
            }
            return Some(Player { health: 100, mana: None, level: self.level });
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => self.no_mana(mana_cost),
            Some(_)=> self.with_mana(mana_cost)
        }
    }

    fn no_mana(&mut self, mana_cost: u32) -> u32 {
        let health_res = match self.health.checked_sub(mana_cost) {
            Some(result) if result > 0 => result,
            Some(_) => 0,
            None => 0
        };
        self.health = health_res;
        0
    }

    fn with_mana(&mut self, mana_cost: u32) -> u32 {
        if mana_cost > self.mana.unwrap() {
            return 0
        }
        self.mana = Some(self.mana.unwrap() - mana_cost);
        mana_cost * 2  
    }
}
