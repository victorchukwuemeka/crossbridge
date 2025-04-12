import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BridgeProgram } from "../target/types/bridge_program";

describe("bridge-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BridgeProgram as Program<BridgeProgram>;

  try {
    it("Is initialized!", async () => {
      // Add your test here.
      //const tx = await program.methods.initialize().rpc();
      //console.log("Your transaction signature", tx);

      const [bridgeAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('bridge_vault')],
        program.programId
      );

      await program.methods.initialize()
      .accounts({ bridgeAccount })
      .rpc()
      console.log("Bridge PDA:", bridgeAccount.toBase58());
    });
  } catch (error) {
    console.error("❌ Initialize failed:", error);
  }

  it("Lock Sol", async () => {
    await program.methods.lockSol(
      new anchor.BN(1_000_000_00),
    )
    .accounts({
      bridgeAccount :bridgePDa,
      user : provider.wallet.PublicKey,
    })
    .rpc();
    
  });
  
});
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BridgeProgram } from "../target/types/bridge_program";
import { assert } from "chai";

describe("bridge-program", () => {
  // 1. Setup Provider
  const provider = anchor.AnchorProvider.local(); // Better for testing than .env()
  anchor.setProvider(provider);

  // 2. Load Program
  const program = anchor.workspace.BridgeProgram as Program<BridgeProgram>;

  // 3. Declare variables we'll reuse
  let bridgePda;
  let user = provider.wallet.publicKey;

  // 4. Initialize Test
  it("Initialize Bridge Account", async () => {
    // Calculate PDA (matches Rust program)
    [bridgePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('bridge_vault')],
      program.programId
    );

    // Call initialize instruction
    const tx = await program.methods.initialize()
      .accounts({ bridgeAccount: bridgePda })
      .rpc();

    console.log("✅ Initialized. PDA:", bridgePda.toBase58(), "TX:", tx);

    // Verify initialization
    const account = await program.account.bridgeAccount.fetch(bridgePda);
    assert.equal(account.totalLocked.toNumber(), 0, "Should start with 0 locked");
  });

  // 5. Lock SOL Test
  it("Lock SOL into Bridge", async () => {
    const amount = new anchor.BN(1_000_000_000); // 1 SOL

    const tx = await program.methods.lockSol(amount)
      .accounts({
        bridgeAccount: bridgePda, // Reuse the PDA
        user: user               // Test wallet
      })
      .rpc();

    console.log("✅ Locked", amount.toNumber()/1e9, "SOL. TX:", tx);

    // Verify lock
    const account = await program.account.bridgeAccount.fetch(bridgePda);
    assert.equal(
      account.totalLocked.toNumber(),
      amount.toNumber(),
      "Locked amount should match"
    );
  });
});