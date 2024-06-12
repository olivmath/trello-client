//! # CLI Tool for management boards on Trello
//!
//! ## Set up your `.env`
//!
//! ```bash
//! TOKEN=
//! API_KEY=
//! SECRET_KEY=
//! BASE_URL=
//! BOARD_ID=
//! ```
//!
//! ## Make simple CRUD
//!
//! ### CREATE
//!
//! - add a card to the board
//!
//! ```bash
//! tc add card \
//!     --name my_card \
//!     --label my_label \
//!     --step my_step
//! ```
//!
//! ### READ
//!
//! - get all cards from the board
//!
//! ```bash
//! tc get card my_board --all
//! ```
//!
//! - get all cards from the board by step
//!
//! ```bash
//! tc get card my_board --step my_steps
//! ```
//!
//! - get a card from the board
//!
//! ```bash
//! tc get card my_board $CARD_NAME
//! tc get card my_board --id $CARD_ID
//! ```
//!
//! ### UPDATE
//!
//! - move card from the board forward
//!
//! ```bash
//! tc move card my_board next --id $CARD_ID
//! tc move card my_board back --id $CARD_ID
//!
//! - update card from the board
//!
//! ```bash
//! tc edit card my_board \
//!     --id $CARD_ID \
//!     --name new_name \
//!     --labe new_label \
//!     --color new_color \
//!     -- ...
//! ```
//!
//! ### DELETE
//!
//! ```bash
//! tc remove card my_board $CARD_NAME
//! tc remove card my_board --id $CARD_ID
//! ```
//!
//! ## Make a massive CRUD
//!
//! - add several cards from json
//!
//! ```bash
//! tc add card my_board --from $JSON_PATH
//! ```
//!
//! - update several cards from json
//!
//! ```bash
//! tc edit card my_board --from $JSON_PATH
//! ```
//!
//! - delete several cards from json
//!
//! ```bash
//! tc remove card my_board --from $JSON_PATH
//! ```
//!

pub mod commands;
pub mod core;
