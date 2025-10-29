# CrossBridge
**Bridging Your SOL to ETH and Beyond**

CrossBridge is a cross-chain bridge enabling secure token transfers from Solana to Ethereum and other EVM-compatible chains.

---

## Project Status

### ✅ Version 1.0 (Working Prototype)
We have a **fully functional bridge** where:
- SOL is locked on Solana
- wSOL is minted on the destination chain (Ethereum/Base/etc.)
- Runs on a centralized relayer with a single signer (not multisig)
- **Status:** Live and operational

### 🚧 Version 2.0 (Trustless - In Development)
We are building the **next generation trustless bridge** featuring:
- **Merkle Proof Verification:** Proves transaction inclusion in Solana blocks
- **Zero-Knowledge Proofs:** ZK proofs generated via SP1 zkVM
- **Cryptographic Verification:** Destination chain verifier contract validates ZK proofs before minting
- **Decentralized:** No reliance on centralized signers or multisig committees

**Workflow:**

Lock SOL on Solana
↓
Generate Merkle Proof (transaction in block)
↓
Pass to SP1 zkVM (generate ZK proof)
↓
Submit ZK proof to destination chain
↓
Verifier contract validates proof
↓
Mint wSOL to user



---

## Features

### Current (V1)
- **SOL → ETH Transfers:** Seamlessly move Solana tokens to Ethereum
- **Real-Time Monitoring:** Track transactions through the bridging process
- **Multi-Chain Ready:** Supports Ethereum, Base, and other EVM chains

### Coming Soon (V2)
- **Trustless Architecture:** Pure cryptographic verification, no trusted intermediaries
- **ZK-Powered Security:** Zero-knowledge proofs ensure transaction validity
- **Merkle Proofs:** Cryptographically prove transactions exist in Solana blocks
- **Developer APIs:** Tools for dApp integration

---

## How It Works

### Version 1.0 (Current)
1. **Lock SOL:** User locks tokens in Solana contract
2. **Event Detection:** Centralized relayer monitors lock events
3. **verify event with pda state 
4. **Sign & Submit:** Relayer signs transaction and submits to destination
5. **Mint wSOL:** Destination contract mints wrapped tokens

### Version 2.0 (Trustless - In Development)
1. **Lock SOL:** User locks tokens on Solana blockchain
2. **Wait for Finality:** System waits for block finalization (32+ slots)
3. **Generate Merkle Proof:** Build cryptographic proof that transaction exists in block
4. **ZK Proof Generation:** SP1 zkVM wraps Merkle proof in zero-knowledge proof
5. **Submit to Destination:** Relayer (anyone can run) submits ZK proof
6. **Verify & Mint:** Verifier contract validates ZK proof and mints wSOL

---

## Current Development Phase

### Phase 1: Infrastructure ✅
- Solana block fetching
- Transaction parsing
- Event monitoring
- Passing the transaction to my merkle prove 

### Phase 2: Merkle Proofs 🚧
- Building Merkle trees from Solana blocks
- Generating transaction inclusion proofs
- **Status:** Implementation complete, testing in progress

### Phase 3: ZK Integration 🚧
- Setting up SP1 zkVM framework
- Designing ZK circuits for proof verification
- Porting Merkle verification to ZK-compatible code
- **Status:** Framework installed, circuit design underway

### Phase 4: Smart Contracts (Next)
- ZK verifier contract on destination chains
- Enhanced lock/unlock mechanisms
- Replay protection

### Phase 5: Testnet & Audits (Upcoming)
- Deploy V2 to testnets
- Security audits
- Bug bounty program

---

## Architecture

### V1 (Current)


Solana → Centralized Relayer → EVM Chain
(Single Signer)

---

## Roadmap

### 2025 Q4
- ✅ Complete Merkle proof generation
- 🚧 Integrate SP1 zkVM
- 🚧 Build ZK circuits for verification

### 2025 Q3-Q4
- Deploy trustless version to testnet
- Security audits
-
