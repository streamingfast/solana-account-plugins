use crate::pb::sf::solana::r#type::v1::{AccountBlock, Block};
use crate::state::BlockInfo;
use crate::state::{ACC_MUTEX, BLOCK_MUTEX};
use base64;
use log::{debug, info};
use prost::Message;
use std::fs::File;
use std::io::Write;

pub struct BlockPrinter {
    noop: bool,
    out_block: File,
    out_account: File,
}

impl BlockPrinter {
    pub fn new(out_block: File, out_account: File, noop: bool) -> Self {
        BlockPrinter {
            noop,
            out_block,
            out_account,
        }
    }

    pub fn print_init(
        &mut self,
        block_type: &str,
        account_block_type: &str,
    ) -> std::io::Result<()> {
        if self.noop {
            debug!(
                "printing init for type {} and {} (noop mode)",
                block_type, account_block_type
            );
            Ok(())
        } else {
            if let Err(e) = writeln!(self.out_block, "FIRE INIT 3.0 {block_type}") {
                return Err(e);
            }
            if let Err(e) = writeln!(self.out_account, "FIRE INIT 3.0 {account_block_type}") {
                return Err(e);
            }
            Ok(())
        }
    }

    pub fn print(
        &mut self,
        block_info: &BlockInfo,
        lib: u64,
        block: Block,
        account_block: AccountBlock,
    ) -> std::io::Result<()> {
        let mut out_block = self.out_block.try_clone().unwrap();
        let slot = block_info.slot;
        let parent_slot = block_info.parent_slot;
        let timestamp_nano = block_info.timestamp.seconds * 1_000_000_000;
        let lib = lib;
        let block_hash = block_info.block_hash.clone();
        let parent_hash = block_info.parent_hash.clone();

        let noop = self.noop;
        std::thread::spawn(move || {
            let encoded_block = block.encode_to_vec();
            let base64_encoded_block = base64::encode(encoded_block);
            let payload = base64_encoded_block;

            if noop {
                info!("printing block {} (noop mode)", slot);
            } else {
                let _lock = BLOCK_MUTEX.lock().unwrap();
                writeln!(out_block, "FIRE BLOCK {slot} {block_hash} {parent_slot} {parent_hash} {lib} {timestamp_nano} {payload}").unwrap()
            }
        });

        let mut out_account = self.out_account.try_clone().unwrap();
        let block_hash2 = block_info.block_hash.clone();
        let parent_hash2 = block_info.parent_hash.clone();
        std::thread::spawn(move || {
            let encoded_account_block = account_block.encode_to_vec();
            let base64_encoded_block = base64::encode(encoded_account_block);
            let payload = base64_encoded_block;
            if noop {
                info!("printing block {} (noop mode)", slot);
            } else {
                let _lock = ACC_MUTEX.lock().unwrap();
                writeln!(out_account, "FIRE BLOCK {slot} {block_hash2} {parent_slot} {parent_hash2} {lib} {timestamp_nano} {payload}").unwrap()
            }
        });

        // We are not waiting for the threads to finish, so that the plugin can be called again for the updates. The lock is only used to prevent interleaving of the output.
        // If an error occurs while writing, the unwrap() will make it panic and poison the mutex.
        // TODO: updating the cursor should be done with that knowledge (maybe wrapping the cursor in the mutex?)
        Ok(())
    }
}
