use crate::*;

pub fn initialize_user(client: &Client, user: Pubkey) -> ClientResult<()> {
    let initiallize_user_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: sol_chess::instruction::InitializeUser {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [initiallize_user_ix].to_vec(),
        None,
        "initialize_user".to_string(),
    )?;

    Ok(())
}
