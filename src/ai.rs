pub struct BookingAI {
    room_database: Vec<Room>,
}

#[derive(Clone)]
pub struct Room {
    pub number: u32,
    pub room_type: String,
    pub price: u32,
    pub available: bool,
}

impl BookingAI {
    pub fn new() -> Self {
        let rooms = vec![
            Room { number: 101, room_type: "quiet".to_string(), price: 50, available: true },
            Room { number: 201, room_type: "view".to_string(), price: 80, available: true },
            Room { number: 301, room_type: "budget".to_string(), price: 30, available: true },
        ];
        Self { room_database: rooms }
    }
    
    pub fn suggest_room(&self, preferences: &str) -> Option<u32> {
        let pref_lower = preferences.to_lowercase();
        
        for room in &self.room_database {
            if room.available && pref_lower.contains(&room.room_type) {
                return Some(room.number);
            }
        }
        
        // Default to first available room
        self.room_database.iter()
            .find(|room| room.available)
            .map(|room| room.number)
    }
    
    pub fn get_room_info(&self, room_number: u32) -> Option<&Room> {
        self.room_database.iter().find(|room| room.number == room_number)
    }
}