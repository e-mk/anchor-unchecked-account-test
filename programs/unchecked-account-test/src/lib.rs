pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;


declare_id!("3B9RuBhQWREjmgeUZJRb18fwLNrqXL9BSBgQ22vnEy79");

#[program]
pub mod unchecked_account_test {

    use super::*;

    pub fn initialize(
      ctx: Context<Initialize>, 
      data_source_name: String, 
      asset_id: String, 
      timestamp: i64) -> Result<()> {
  
        let data = ctx.accounts.oracle_account.try_borrow_mut_data()?;
        let account = PriceUpdateV2::try_deserialize(&mut data.as_ref()).expect("Error deserializing account data");
  
      
      // &ctx.accounts.record_price(data_source_name, asset_id, timestamp);
  
      Ok(())
    
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  pub oracle_account: UncheckedAccount<'info>,
}
