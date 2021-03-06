use anchor_lang::prelude::*;

declare_id!("HJqSGkR5hP5d51ywhYkuwGYF4Tj3aL8A6wNS9Q5dHrM");

#[program]
pub mod counterapp {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space = 16+16)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterAccount {
    pub count: u64,
}

#[account]
pub struct CompanyAccount {
    pub company_id: u64,   // incremental id
    pub created_time: u64, // date when company is created
    pub claim_date: u64 // date when user claim
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}
