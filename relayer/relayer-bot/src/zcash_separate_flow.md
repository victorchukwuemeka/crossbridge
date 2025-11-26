Here is a clean, well-formatted **Markdown rewrite** of your explanation:

---

# `LockEvent` (Solana Program Event)

```rust
#[event]
pub struct LockEvent {
    pub user: Pubkey,              // Who locked
    pub eth_address: String,       // Address to mint wSOL on EVM
    pub amount: u64,               // Amount of SOL (lamports)
    pub fee: u64,                  // Bridge fee
    pub target_network: u8,        // Destination EVM chain ID
    pub timestamp: i64,            // Block time
    pub commitment: [u8; 32],      // Unique lock ID
    pub privacy_enabled: bool,     // Whether Zcash privacy is used
}
```

---

# **How the Event Connects to Zcash Functions**

## **Relayer Workflow (Event Listener → Zcash → EVM)**

```
Relayer Listens to Solana WebSocket
│
├─ Detects LockEvent from your program
│
├─ Extracts:
│     commitment        ← event.commitment
│     amount            ← event.amount
│     eth_address       ← event.eth_address
│     privacy_enabled   ← event.privacy_enabled
│
├─ IF privacy_enabled == true:
│
│   1. create_shielded_address()
│      → Generate Zcash shielded address
│
│   2. build_shield_transaction(
│         amount: event.amount,
│         memo:   event.commitment
│      )
│      → Deposit into Zcash pool
│
│   3. broadcast_transaction(tx)
│      → Send Zcash deposit to network
│
│   4. track_commitment_to_zcash_tx(
│         commitment: event.commitment,
│         txid: zcash_txid
│      )
│      → Save mapping in database
│
│   5. wait_for_confirmations(txid, 10)
│      → Ensure Zcash deposit is final
│
│   6. wait_for_privacy_delay(20 blocks)
│      → Allow mixing for privacy
│
│   7. query_shielded_notes()
│      → Find shielded note we deposited
│
│   8. build_unshield_transaction(note)
│      → Withdraw note from pool
│
│   9. broadcast_transaction(tx)
│      → Send withdrawal transaction
│
│  10. mint_on_evm(
│         recipient: event.eth_address,
│         amount:    event.amount
│      )
│      → Mint wSOL on EVM chain
│
└─ ELSE (privacy_enabled == false):
       → Skip Zcash entirely
       → mint_on_evm(event.eth_address, event.amount)
```

---

