use hostel_booking_ai::{Booking, BookingAI};

fn main() {
    println!("🏨 Welcome to Hostel Booking AI System!");
    println!("=========================================\n");
    
    let ai = BookingAI::new();
    
    // Test different preferences
    let test_cases = vec![
        ("Bonface Too", "quiet room please"),
        ("Juliet Chelugui", "room with a view"),
        ("Brendah Chelugui", "budget option"),
    ];
    
    for (i, (guest_name, preferences)) in test_cases.iter().enumerate() {
        println!("📋 Processing booking #{}", i + 1);
        println!("Guest: {}", guest_name);
        println!("Preferences: {}", preferences);
        
        match ai.suggest_room(preferences) {
            Some(room_number) => {
                let booking = Booking::new(
                    (i + 1) as u32,
                    guest_name.to_string(),
                    room_number,
                    "2026-01-01".to_string(),
                    "2026-01-03".to_string(),
                );
                
                if let Some(room_info) = ai.get_room_info(room_number) {
                    println!("✅ Booking confirmed!");
                    println!("   Room: {} ({})", room_number, room_info.room_type);
                    println!("   Price: KSH{}/night", room_info.price);
                    println!("   Guest: {}", booking.guest_name);
                }
            }
            None => {
                println!("❌ No rooms available for preferences: {}", preferences);
            }
        }
        println!();
    }
    
    println!("🎉 Hostel Booking AI Demo Complete!");
}
