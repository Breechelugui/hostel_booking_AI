pub struct Booking {
    pub id: u32,
    pub guest_name: String,
    pub room_number: u32,
    pub check_in: String,
    pub check_out: String,
}

impl Booking {
    pub fn new(id: u32, guest_name: String, room_number: u32, check_in: String, check_out: String) -> Self {
        Self { id, guest_name, room_number, check_in, check_out }
    }
}