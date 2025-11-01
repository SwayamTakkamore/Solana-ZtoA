use anchor_lang::prelude::*;

declare_id!("AoiqMWavS4Cza944m9QFhz42HtfWFwbnwmhwAfcQ3yY8");

#[program]
pub mod gm_swayam {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
