use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_client::{thread, Client, ClientResult},
    solana_sdk::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        system_program, sysvar::clock,
    },
};

pub mod contexts;
pub mod utils;

pub use contexts::*;
pub use utils::*;

fn main() -> ClientResult<()> {
    let payer = Keypair::new();
    let client = Client::new(payer, "https://api.devnet.solana.com".into());
    client.airdrop(&client.payer_pubkey(), 1 * LAMPORTS_PER_SOL)?;

    let user = sol_chess::User::pda(client.payer_pubkey()).0;
    let game = sol_chess::Game::pda(user.key(), 0).0;

    initialize_user(&client, user)?;
    initialize_game(&client, user, game, Some(1000), 60, 5, true)?;

    deposit(&client, user, 100000)?;

    join_game(&client, user, game, sol_chess::Color::White)?;
    join_game(&client, user, game, sol_chess::Color::Black)?;
    let from = sol_chess::Square { file: 0, rank: 6 };
    let to = sol_chess::Square { file: 0, rank: 5 };
    move_piece(&client, user, game, from, to)?;
    let from = sol_chess::Square { file: 0, rank: 1 };
    let to = sol_chess::Square { file: 0, rank: 2 };
    move_piece(&client, user, game, from, to)?;

    let mut data: &[u8] = &client.get_account_data(&game).unwrap();
    let game_account = sol_chess::Game::try_deserialize(&mut data).unwrap();
    println!("{:?}", game_account.time_control);

    let from = sol_chess::Square { file: 1, rank: 6 };
    let to = sol_chess::Square { file: 1, rank: 5 };
    move_piece(&client, user, game, from, to)?;
    let from = sol_chess::Square { file: 1, rank: 1 };
    let to = sol_chess::Square { file: 1, rank: 2 };
    move_piece(&client, user, game, from, to)?;
    resign(&client, user, user, game)?;

    let mut data: &[u8] = &client.get_account_data(&game).unwrap();
    let game_account = sol_chess::Game::try_deserialize(&mut data).unwrap();
    println!("{:?}", game_account.time_control);

    Ok(())
}
