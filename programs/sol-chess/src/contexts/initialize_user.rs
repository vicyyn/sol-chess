use crate::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,payer=payer,space=8 + size_of::<User>(), seeds=[SEED_USER,payer.key().as_ref()], bump)]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn process(&mut self) -> Result<()> {
        let InitializeUser { user, .. } = self;
        user.new()?;
        Ok(())
    }
}
