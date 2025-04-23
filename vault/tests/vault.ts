import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  const connection = provider.connection;

  let signer;
  let vault_state;
  let bump;
  let vault;


  before(
    async () => {
      signer = anchor.web3.Keypair.generate();

      [vault_state, bump] = PublicKey.findProgramAddressSync([
        Buffer.from("state"),
        signer.publicKey.toBuffer(),        //pda jisse derive hua hai, uska toBuffer karna hai
      ], program.programId);

      [vault, bump] = PublicKey.findProgramAddressSync([
        vault_state.toBuffer(),
      ], program.programId);

      await airdrop(connection, signer.publicKey, 5);

      // to make the vault rent-exempt
      await airdrop(connection, vault, 1);
    }  
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .accounts({
      signer: signer.publicKey,
      vaultState: vault_state,
      vault: vault,
      systemProgram: SystemProgram.programId,
    })
    .signers([signer])
    .rpc();
    console.log("Your initialize transaction signature", tx);
  });

  it("Is Deposited!", async () => {
    const tx = await program.methods
    .deposit(new BN(1_000_000_000))
    .accounts({
      signer: signer.publicKey,
      vaultState: vault_state,
      vault: vault,
      systemProgram: SystemProgram.programId
    })
    .signers([signer])
    .rpc();
    console.log("Your deposit transaction signature", tx);


    //checks
    let vault_balance = await getBalance(connection, vault);
    assert.equal(vault_balance, 2 * 1_000_000_000);   //provided by chai
    
    let signerBalance = await getBalance(connection, signer.publicKey); 
    assert.isTrue(signerBalance >= 5);
  })

  it("Is Withdrawn!", async () => {
    const tx = await program.methods
    .withdraw(new BN(1))
    .accounts({
      signer: signer.publicKey,
      vaultState: vault_state,
      vault: vault,
      systemProgram: SystemProgram.programId
    })
    .signers([signer])
    .rpc();

    console.log("Your withdraw transaction signature", tx);
  })

  it("Is Closed!", async () => {
    const tx = await program.methods
    .close()
    .accounts({
      signer: signer.publicKey,
      vaultState: vault_state,
      vault: vault,
      systemProgram: SystemProgram.programId
    })
    .signers([signer])
    .rpc;
  })
});

async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}