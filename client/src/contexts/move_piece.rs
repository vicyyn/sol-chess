use crate::*;

pub fn move_piece(
    client: &Client,
    user: Pubkey,
    game: Pubkey,
    from: sol_chess::Square,
    to: sol_chess::Square,
) -> ClientResult<()> {
    let move_piece_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(game, false),
        ],
        data: sol_chess::instruction::MovePiece { from, to }.data(),
    };

    send_and_confirm_tx(
        &client,
        [move_piece_ix].to_vec(),
        None,
        "move_piece".to_string(),
    )?;

    Ok(())
}
