use anchor_lang::prelude::*;
pub mod state;
pub mod context;
pub mod helper;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
use context::*;
use state::*;
use helper::*;
#[program]
pub mod multisig_sol {
  use super::*;
  pub fn initialize(ctx: Context<InitializeContext>, _appdata: Appdata) -> Result<()> {
    let appdata = &mut ctx.accounts.appdata;
    appdata.voters = _appdata.voters;
    appdata.threshold = _appdata.threshold;
    appdata.total_weight = _appdata.total_weight;
    appdata.proposal_count = _appdata.proposal_count;

    Ok(())
  }

  pub fn create_proposal(ctx: Context<CreateProposalContext>, _proposal: NewProposal) -> Result<()> {
    let appdata = &mut ctx.accounts.appdata;
    let proposer = &mut ctx.accounts.signer;
    is_voter(appdata, proposer.key)?;

    let proposal = Proposal {
      proposal_id: appdata.proposal_count,
      proposer: proposer.key(),
      name: _proposal.name,
      description: _proposal.description,
      start: _proposal.start,
      end: _proposal.end,
      yes: 0,
      no: 0,
      threshold: appdata.threshold,
      total_weight: ctx.accounts.appdata.total_weight,
      status: ProposalStatus::Open,
      action: _proposal.action,
    };

    let proposals = &mut ctx.accounts.proposals;
    proposals.opening_proposals.push(proposal);

    Ok(())
  }
}