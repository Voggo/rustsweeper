//! # Rustsweeper
//!
//! A minimal terminal-based Minesweeper game written in Rust, using `crossterm` for cross-platform terminal UI.
//!
//! ## Features
//! - Classic Minesweeper gameplay
//! - Mouse controls
//! - Multiple difficulty levels and custom boards
//! - Colorful terminal UI
//!
//! ## Usage
//! See the README for instructions on running the game as an application.
//!
//! ## Modules
//! - [`game_logic`] - Core game logic and board state
//! - [`menu`] - Menu system and event handling
//! - [`timer`] - Simple timer for tracking game duration
//! - [`tui`] - Terminal UI rendering
//! - [`types`] - Common types and configuration

/// Core game logic and board state.
pub mod game_logic;
/// Menu system and event handling.
pub mod menu;
/// Simple timer for tracking game duration.
pub mod timer;
/// Terminal UI rendering.
pub mod tui;
/// Common types and configuration.
pub mod types;
