#[event]
pub struct LockEvent {
    pub user: Pubkey,              // Who locked
    pub eth_address: String,       // Where to mint wSOL
    pub amount: u64,               // How much SOL (in lamports)
    pub fee: u64,                  // Fee taken
    pub target_network: u8,        // Which EVM chain
    pub timestamp: i64,            // When locked
    pub commitment: [u8; 32],      // Unique ID for this lock
    pub privacy_enabled: bool,     // Use Zcash or not
}
```

---

## **HOW EVENT CONNECTS TO ZCASH FUNCTIONS:**
```
EVENT LISTENER (Relayer)
│
├─→ Listens to Solana WebSocket
├─→ Filters for YOUR program's events
├─→ When LockEvent detected:
│
│   Extract Data:
│   ├─→ commitment = event.commitment
│   ├─→ amount = event.amount
│   ├─→ eth_address = event.eth_address
│   ├─→ privacy_enabled = event.privacy_enabled
│   
│   IF privacy_enabled == true:
│   │
│   │   Call Zcash Functions:
│   │   
│   │   1. create_shielded_address()
│   │      └─→ Generate where to deposit
│   │
│   │   2. build_shield_transaction(
│   │         amount: event.amount,      ← FROM EVENT
│   │         memo: event.commitment     ← FROM EVENT
│   │      )
│   │      └─→ Create deposit tx
│   │
│   │   3. broadcast_transaction(tx)
│   │      └─→ Send to Zcash network
│   │
│   │   4. track_commitment_to_zcash_tx(
│   │         commitment: event.commitment,  ← FROM EVENT
│   │         txid: zcash_txid
│   │      )
│   │      └─→ Save to database
│   │
│   │   5. wait_for_confirmations(txid, 10)
│   │      └─→ Wait for Zcash confirmations
│   │
│   │   6. wait_for_privacy_delay(20 blocks)
│   │      └─→ Let it mix in pool
│   │
│   │   7. query_shielded_notes()
│   │      └─→ Find the note we deposited
│   │
│   │   8. build_unshield_transaction(note)
│   │      └─→ Withdraw from pool
│   │
│   │   9. broadcast_transaction(tx)
│   │      └─→ Send withdrawal
│   │
│   │   10. mint_on_evm(
│   │          recipient: event.eth_address,  ← FROM EVENT
│   │          amount: event.amount           ← FROM EVENT
│   │       )
│   │       └─→ User gets wSOL
│   
│   ELSE (privacy_enabled == false):
│   │
│   └─→ Skip Zcash, directly mint_on_evm()
