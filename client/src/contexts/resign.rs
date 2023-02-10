use crate::*;

pub fn resign(
    client: &Client,
    user: Pubkey,
    adversary_user: Pubkey,
    game: Pubkey,
) -> ClientResult<()> {
    let resign_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(adversary_user, false),
            AccountMeta::new(game, false),
        ],
        data: sol_chess::instruction::Resign {}.data(),
    };

    send_and_confirm_tx(&client, [resign_ix].to_vec(), None, "resign".to_string())?;

    Ok(())
}
