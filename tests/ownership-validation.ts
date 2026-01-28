import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OwnershipValidation } from "../target/types/ownership_validation";
import { expect } from "chai";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";

describe("🛡️ Security Module: Ownership Validation", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OwnershipValidation as Program<OwnershipValidation>;
  const provider = anchor.getProvider();

  // Accounts for testing
  const configAccount = Keypair.generate();
  const admin = (provider.wallet as anchor.Wallet).payer;
  const fakeAdmin = Keypair.generate();

  it("✅ Setup: Create a legitimate Config account", async () => {
    await program.methods
      .initialize(admin.publicKey)
      .accounts({
        config: configAccount.publicKey,
        user: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([configAccount])
      .rpc();

    const account = await program.account.config.fetch(configAccount.publicKey);
    expect(account.admin.toBase58()).to.equal(admin.publicKey.toBase58());
  });

  it("❌ SECURE: Should block update if non-admin tries to update (has_one check)", async () => {
    try {
      await program.methods
        .updateAdminSecure(fakeAdmin.publicKey)
        .accounts({
          config: configAccount.publicKey,
          admin: fakeAdmin.publicKey,
        })
        .signers([fakeAdmin])
        .rpc();

      expect.fail("The transaction should have failed");
    } catch (e) {
      // Anchor throws a ConstraintHasOne error
      expect(e.toString()).to.include("A has one constraint was violated");
    }
  });

  it("🔴 VULNERABLE: Demonstrating the exploit (Conceptual)", async () => {
    console.log("   > In a real exploit, the attacker would pass an account owned by a MALICIOUS program.");
    console.log("   > Because `update_admin_vulnerable` uses `AccountInfo` without checking `owner`, it would trust it.");
    // We can't easily deploy a second malicious program in a single `anchor test` run without more setup,
    // but we can show that passing a DIFFERENT account type (like SystemProgram) would fail 
    // in the SECURE version but potentially be mis-deserialized in the VULNERABLE version.
  });

  it("✅ SECURE: Successfully update admin with correct signature", async () => {
    const newAdmin = Keypair.generate();
    await program.methods
      .updateAdminSecure(newAdmin.publicKey)
      .accounts({
        config: configAccount.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const account = await program.account.config.fetch(configAccount.publicKey);
    expect(account.admin.toBase58()).to.equal(newAdmin.publicKey.toBase58());
  });
});
