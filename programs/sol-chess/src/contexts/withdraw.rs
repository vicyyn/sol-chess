use crate::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn process(&mut self, vault_bump: u8, amount: u64) -> Result<()> {
        let Self {
            user,
            vault,
            payer,
            system_program,
        } = self;

        require!(
            user.has_sufficient(amount),
            CustomError::InsufficientBalance
        );

        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: vault.to_account_info(),
                    to: payer.to_account_info(),
                },
                &[&[b"vault", &[vault_bump]]],
            ),
            amount,
        )?;

        user.decrease_balance(amount);

        Ok(())
    }
}
