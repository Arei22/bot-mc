#![deny(clippy::correctness, clippy::nursery, clippy::pedantic, clippy::all)]
#![allow(
    clippy::missing_errors_doc,
    clippy::unused_async,
    clippy::missing_panics_doc,
    clippy::future_not_send,
    clippy::too_many_lines,
    clippy::manual_let_else,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal
)]

pub mod client;
pub mod commands;
pub mod database;
pub mod interarction;
pub mod util;
