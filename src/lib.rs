use hex_literal::hex;
use pb::eth::lpp::v1 as lpp;
use substreams::{Hex, log, store::StoreAddInt64};
use substreams::prelude::*;
use substreams_ethereum::{NULL_ADDRESS, pb::eth::v2 as eth};

mod abi;
mod pb;

const LENS_PROTOCOL_CONTRACT: [u8; 20] = hex!("Db46d1Dc155634FbC732f92E853b10B288AD5a1d");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<lpp::Transfers, substreams::errors::Error> {
    Ok(lpp::Transfers {
        transfers: blk
            .events::<abi::lpp::events::Transfer>(&[&LENS_PROTOCOL_CONTRACT])
            .map(|(transfer, log)| {
                substreams::log::info!("LPP transfer");

                lpp::Transfer {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    from: transfer.from,
                    to: transfer.to,
                    token_id: transfer.token_id.to_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the total number of LPP transfers by holder
#[substreams::handlers::store]
fn store_transfers(transfers: lpp::Transfers, s: StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

/// Extracts approval events from the contract
#[substreams::handlers::map]
fn map_approvals(blk: eth::Block) -> Result<lpp::Approvals, substreams::errors::Error> {
    Ok(lpp::Approvals {
        approvals: blk
            .events::<abi::lpp::events::Approval>(&[&LENS_PROTOCOL_CONTRACT])
            .map(|(approval, log)| {
                substreams::log::info!("LPP approval");

                lpp::Approval {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    owner: approval.owner,
                    approved: approval.approved,
                    token_id: approval.token_id.to_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the total number of LPP approvals by holder
#[substreams::handlers::store]
fn store_approvals(approvals: lpp::Approvals, s: StoreAddInt64) {
    for approval in approvals.approvals {
        log::info!("Found approval {}", Hex(&approval.trx_hash));
        s.add(approval.ordinal, generate_key(&approval.owner), -1);
    }
}


fn generate_key(holder: &Vec<u8>) -> String {
    return format!("total:{}:{}", Hex(holder), Hex(LENS_PROTOCOL_CONTRACT));
}
