use anchor_lang::{ prelude::* };

#[account]
pub struct Appdata {
  pub voters: Vec<Voter>,
  pub threshold: Threshold,
  pub total_weight: u64,
  pub proposal_count: u64,
}

impl Appdata {
  pub fn size(appdata: Appdata) -> usize {
    appdata.voters.len() * 32 + 8 + 8 + 8 + 8
  }
}

#[account]
pub struct Proposals {
  pub opening_proposals: Vec<Proposal>,
  pub closed_proposals: Vec<Proposal>,
  pub executed_proposals: Vec<Proposal>,
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub struct Voter {
  pub addr: Pubkey,
  pub weight: u64,
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum Threshold {
  Absolute(u64),
  Percent(u64),
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Proposal {
  pub proposal_id: u64,
  pub proposer: Pubkey,
  pub name: String,
  pub description: String,
  pub start: i64,
  pub end: i64,
  pub yes: u64,
  pub no: u64,
  pub threshold: Threshold,
  pub total_weight: u64,
  pub status: ProposalStatus,
  pub action: Action,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Action {
  pub program_id: Pubkey,
  // Accounts requried for the transaction.
  pub accounts: Vec<TransactionAccount>,
  // Instruction data for the transaction.
  pub data: Vec<u8>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
  pub pubkey: Pubkey,
  pub is_signer: bool,
  pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
  fn from(account: &TransactionAccount) -> AccountMeta {
    match account.is_writable {
      false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
      true => AccountMeta::new(account.pubkey, account.is_signer),
    }
  }
}

impl From<&AccountMeta> for TransactionAccount {
  fn from(account_meta: &AccountMeta) -> TransactionAccount {
    TransactionAccount {
      pubkey: account_meta.pubkey,
      is_signer: account_meta.is_signer,
      is_writable: account_meta.is_writable,
    }
  }
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum ProposalStatus {
  Open,
  Closed,
  Executed,
  Accepted,
  Expired,
}

impl Proposal {
  pub fn update_status(&mut self) {
    match self.threshold {
      Threshold::Absolute(threshold) => {
        if self.yes >= threshold {
          self.status = ProposalStatus::Accepted;
        } else if self.no >= threshold {
          self.status = ProposalStatus::Closed;
        } else if self.end < Clock::get().unwrap().unix_timestamp {
          self.status = ProposalStatus::Expired;
        }
      }

      Threshold::Percent(threshold) => {
        let total = self.total_weight;
        let accept_percent = ((self.yes as f64) / (total as f64)) * 100.0;
        let reject_percent = ((self.no as f64) / (total as f64)) * 100.0;
        if accept_percent >= (threshold as f64) {
          self.status = ProposalStatus::Accepted;
        } else if reject_percent >= (threshold as f64) {
          self.status = ProposalStatus::Closed;
        } else if self.end < Clock::get().unwrap().unix_timestamp {
          self.status = ProposalStatus::Expired;
        }
      }
    }
  }
}

pub enum Vote {
  Yes,
  No,
  Neutral,
}

pub struct Ballot {
  pub proposal_id: u64,
  pub voter: Pubkey,
  pub vote: Vote,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct NewProposal {
  pub name: String,
  pub description: String,
  pub start: i64,
  pub end: i64,
  pub action: Action,
}