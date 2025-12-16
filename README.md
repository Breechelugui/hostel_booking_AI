# Hostel Booking ai - Rust Beginner's Toolkit

🏨 A Rust-based ai system demonstrating core language features through a hostel booking application.

## 🎯 Project Overview

This project serves as a **beginner's toolkit for learning Rust** through a practical example. It demonstrates:
- Rust's ownership model and borrowing
- Struct definitions and implementations
- Module organization and visibility
- Pattern matching and Option types
- Basic "ai" logic with preference matching

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ installed ([Install Rust](https://rustup.rs/))
- Git for cloning

### Run the Project
```bash
# Clone the repository
git clone https://github.com/Breechelugui/hostel_booking_ai.git
cd hostel_booking_ai

# Build and run
cargo build
cargo run
```

### Expected Output
```
🏨 Welcome to Hostel Booking ai System!
=========================================

📋 Processing booking #1
Guest: John Doe
Preferences: quiet room please
✅ Booking confirmed!
   Room: 101 (quiet)
   Price: $50/night
   Guest: John Doe

...
```

## 📚 Learning Resources

For the complete learning guide, see [TOOLKIT.md](./TOOLKIT.md) which includes:
- Step-by-step Rust setup
- Code explanations with ai prompts used
- Common errors and solutions
- Next steps for beginners

## 📜 Project Structure

```
src/
├── main.rs      # Entry point with demo
├── lib.rs       # Library exports
├── booking.rs   # Booking struct and methods
└── ai.rs        # ai logic and room management
```

## 🧪 Features Demonstrated

- **Ownership & Borrowing**: Safe memory management without garbage collection
- **Structs & Methods**: Object-oriented patterns in Rust
- **Modules**: Code organization and visibility
- **Pattern Matching**: Powerful control flow with `match`
- **Option Types**: Null safety with `Some`/`None`

## 🔧 Built With

- **Rust 1.70+** - Systems programming language
- **Cargo** - Rust's build system and package manager

---

🎓 **Part of Moringa ai Capstone Project**: *Prompt-Powered Kickstart for Rust Programming*