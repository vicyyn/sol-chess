use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_client::{Client, ClientResult},
    solana_sdk::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        system_program,
    },
};

pub mod contexts;
pub mod utils;

pub use contexts::*;
pub use utils::*;

fn main() -> ClientResult<()> {
    let payer = Keypair::new();
    let client = Client::new(payer, "http://localhost:8899".into());
    client.airdrop(&client.payer_pubkey(), 1 * LAMPORTS_PER_SOL)?;

    let user = sol_chess::User::pda(client.payer_pubkey()).0;
    let game = sol_chess::Game::pda(client.payer_pubkey(), 0).0;

    initialize_user(&client, user)?;
    initialize_game(&client, user, game)?;
    join_game(&client, user, game, sol_chess::Color::White)?;
    join_game(&client, user, game, sol_chess::Color::Black)?;

    Ok(())
}
