use anchor_lang::prelude::*;
use crate::{ state::{ Appdata, Proposals } };

#[instruction(_appdata: Appdata)]
#[derive(Accounts)]
pub struct InitializeContext<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(init, payer = signer, space = Appdata::size(_appdata), seeds = [b"appdata".as_ref()], bump)]
  pub appdata: Account<'info, Appdata>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposalContext<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub appdata: Account<'info, Appdata>,

  #[account(mut)]
  pub proposals: Account<'info, Proposals>,
}

#[derive(Accounts)]
pub struct VoteContext<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account()]
  pub appdata: Account<'info, Appdata>,
}