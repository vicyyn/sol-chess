use crate::*;

pub fn deposit(client: &Client, user: Pubkey, amount: u64) -> ClientResult<()> {
    let vault = Pubkey::find_program_address(&[b"vault"], &sol_chess::ID).0;
    client.airdrop(&vault, 1000000)?;

    let deposit_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: sol_chess::instruction::Deposit { amount }.data(),
    };

    send_and_confirm_tx(&client, [deposit_ix].to_vec(), None, "deposit".to_string())?;

    Ok(())
}
