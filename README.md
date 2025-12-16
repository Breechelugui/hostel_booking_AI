# Hostel Booking AI

🏨 A Rust-based AI system for hostel room booking with smart preference matching.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

A beginner-friendly Rust project demonstrating:
- Ownership and borrowing
- Structs and methods
- Module organization
- Pattern matching
- AI-powered room suggestions

## Quick Start

```bash
# Clone and run
git clone https://github.com/Breechelugui/hostel_booking_ai.git
cd hostel_booking_ai
cargo run
```

**Prerequisites**: [Rust 1.70+](https://rustup.rs/)

## Features

- **Smart Room Matching** 
- AI suggests rooms based on guest preferences
- **Booking Management** 
- Create and track reservations
- **Price Calculation** 
- Dynamic pricing by room type
- **Memory Safe** 
- Rust's ownership system prevents common bugs

## Project Structure

```
src/
├── main.rs      # Entry point and demo
├── lib.rs       # Module exports
├── booking.rs   # Booking logic
├── ai.rs        # AI room suggestions
└── room.rs      # Room management
```

## Example Output

```
🏨 Welcome to Hostel Booking AI System!

📋 Processing booking #1
Guest: Bonface Too
Preferences: quiet room please
✅ Booking confirmed!
   Room: 101 (quiet)
   Price: KSH5000/night
```

## Learning Resources

See [TOOLKIT.md](./TOOLKIT.md) for:
- Complete Rust setup guide
- Code explanations with AI prompts
- Common errors and solutions
- Next learning steps

## Testing

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # With output
```

## Built With

- **Rust** - Systems programming language
- **Cargo** - Build system and package manager
- **Standard Library** - No external dependencies

---

🎓 **Moringa AI Capstone Project** - Learning Rust through AI-assisted development