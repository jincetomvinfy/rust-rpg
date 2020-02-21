use std::time::{SystemTime, UNIX_EPOCH};

pub struct GameContext{
    time : u128
}

 impl GameContext {
    pub fn new()->GameContext{
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        GameContext{ time: since_the_epoch.as_millis() }
    }

     pub fn started_at(&self)->u128{
         self.time
     }
}