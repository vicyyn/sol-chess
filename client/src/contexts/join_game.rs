use crate::*;

pub fn join_game(
    client: &Client,
    user: Pubkey,
    game: Pubkey,
    color: sol_chess::Color,
) -> ClientResult<()> {
    let join_game_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(user, false),
            AccountMeta::new(game, false),
        ],
        data: sol_chess::instruction::JoinGame { color }.data(),
    };

    send_and_confirm_tx(
        &client,
        [join_game_ix].to_vec(),
        None,
        "join_game_ix".to_string(),
    )?;

    Ok(())
}
