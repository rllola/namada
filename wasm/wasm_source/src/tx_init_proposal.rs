//! A tx to create a governance proposal.

use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, tx_data: Tx) -> TxResult {
    let signed = tx_data;
    let data = signed.data().ok_or_err_msg("Missing data")?;
    let tx_data =
        transaction::governance::InitProposalData::try_from_slice(&data[..])
            .wrap_err("failed to decode InitProposalData")?;
    log_string("apply_tx called to create a new governance proposal");

    governance::init_proposal(ctx, tx_data)
}
