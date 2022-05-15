// set spend tx core.
for (i, sigmap) in signatures.iter().enumerate() {
    if sigmap.len() < mans_thresh {
        return Err(CommandError::SpendNotEnoughSig(sigmap.len(), mans_thresh));
    }
    for (pubkey, raw_sig) in sigmap {
        let sig = secp256k1::Signature::from_der(&raw_sig[..raw_sig.len() - 1])
            .map_err(|_| CommandError::SpendInvalidSig(raw_sig.clone()))?;
        spend_tx
            .psbt
            .add_signature(i, pubkey.key, sig, &revaultd.secp_ctx)
            .map_err(|_| CommandError::SpendInvalidSig(raw_sig.clone()))?
            .expect("The signature was already there");
    }
}

// collecting the signatures.
let signatures: Vec<BTreeMap<BitcoinPubKey, Vec<u8>>> = spend_tx
    .psbt
    .psbt()
    .inputs
    .iter()
    .map(|i| i.partial_sigs.clone())
    .collect();
    
// DbSpendTransaction
let old_transaction = db_spend_transaction(&db_path, &spend_txid);


// Old if else condition.
        if db_spend_transaction(&db_path, &spend_txid)
            .expect("Database must be available")
            .is_some()
        {
            log::debug!("Updating Spend transaction '{}'", spend_txid);
            db_update_spend(&db_path, &spend_tx, false).expect("Database must be available");
 
// Merge the partial sigs of two transactions of the same type into the first one
//
// Returns true if this made the transaction "valid" (fully signed).
// MERGE SIGNATURE.
fn revault_txs_merge_sigs<T, S>(tx_a: &mut T, tx_b: &T, secp: &secp256k1::Secp256k1<S>) -> bool


// Merge the signatures for two transactions into the first one
//
// The two transaction MUST be of the same type.
fn db_txs_merge_sigs(
    tx_a: &mut DbTransaction,
    tx_b: &DbTransaction,
    secp: &secp256k1::Secp256k1<impl secp256k1::Verification>,
) -> bool {
