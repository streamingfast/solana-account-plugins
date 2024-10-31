use crate::pb::sf::solana::r#type::v1::{Account, AccountBlock};
use crate::state::{AccountChanges, BlockInfo};
use prost_types::Timestamp as ProstTimestamp;
use solana_program::clock::UnixTimestamp;

pub fn convert_sol_timestamp(sol_timestamp: UnixTimestamp) -> ProstTimestamp {
    let seconds = sol_timestamp as i64;
    ProstTimestamp { seconds, nanos: 0 }
}

pub fn create_account_block(
    account_changes: &AccountChanges,
    block_info: &BlockInfo,
) -> AccountBlock {
    let mut accounts: Vec<Account> = account_changes
        .into_iter()
        .map(|(_account_key, account)| account.account.clone())
        .collect();

    accounts.sort_by(|a, b| a.address.cmp(&b.address));

    AccountBlock {
        slot: block_info.slot,
        hash: block_info.block_hash.clone(),
        parent_hash: block_info.parent_hash.clone(),
        parent_slot: block_info.parent_slot,
        accounts: accounts,
        timestamp: Some(block_info.timestamp.clone()),
    }
}
