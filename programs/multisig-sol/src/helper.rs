use anchor_lang::prelude::{ Pubkey, ProgramError };

use crate::state::Appdata;

pub fn is_voter(appdata: &Appdata, voter: &Pubkey) -> Result<(), ProgramError> {
  let voter = appdata.voters.iter().find(|v| { v.addr == *voter });
  match voter {
    Some(voter) => {
      if voter.weight > 0 { Ok(()) } else { Err(ProgramError::InvalidAccountData) }
    }
    None => Err(ProgramError::InvalidAccountData),
  }
}