use crate::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let Self {
            user,
            vault,
            payer,
            system_program,
            ..
        } = self;

        anchor_lang::system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: payer.to_account_info(),
                    to: vault.to_account_info(),
                },
            ),
            amount,
        )?;

        user.increase_balance(amount);
        Ok(())
    }
}
