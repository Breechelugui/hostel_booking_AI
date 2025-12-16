#[cfg(test)]
mod tests {
    use crate::{Booking, BookingAI};

    #[test]
    fn test_booking_creation() {
        let booking = Booking::new(
            1,
            "Test Guest".to_string(),
            101,
            "2024-01-01".to_string(),
            "2024-01-03".to_string(),
        );
        
        assert_eq!(booking.id, 1);
        assert_eq!(booking.guest_name, "Test Guest");
        assert_eq!(booking.room_number, 101);
    }

    #[test]
    fn test_ai_room_suggestion() {
        let ai = BookingAI::new();
        
        // Test quiet room preference
        let room = ai.suggest_room("I need a quiet room");
        assert_eq!(room, Some(101));
        
        // Test view preference
        let room = ai.suggest_room("room with a view");
        assert_eq!(room, Some(201));
        
        // Test budget preference
        let room = ai.suggest_room("budget option");
        assert_eq!(room, Some(301));
    }

    #[test]
    fn test_room_info_retrieval() {
        let ai = BookingAI::new();
        
        let room_info = ai.get_room_info(101);
        assert!(room_info.is_some());
        
        if let Some(room) = room_info {
            assert_eq!(room.number, 101);
            assert_eq!(room.room_type, "quiet");
            assert_eq!(room.price, 50);
        }
    }
}