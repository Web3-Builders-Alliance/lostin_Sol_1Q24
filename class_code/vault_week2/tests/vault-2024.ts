import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";
import { Vault2024 } from "../target/types/vault_2024";

const program = anchor.workspace.Vault2024 as Program<Vault2024>;

const connection = anchor.getProvider().connection;

const maker = Keypair.generate();
const taker = Keypair.generate();
const seed = new anchor.BN(1);

const vault = PublicKey.findProgramAddressSync(
  [
    Buffer.from("vault"),
    seed.toBuffer("le", 8),
    maker.publicKey.toBuffer(),
    taker.publicKey.toBuffer(),
  ],
  program.programId
)[0];

const confirm = async (signature: string): Promise<string> => {
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature,
    ...block,
  });
  return signature;
};

const log = async (signature: string): Promise<string> => {
  console.log(
    `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
  );
  return signature;
};

it("Airdrop", async () => {
  await connection
    .requestAirdrop(maker.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log);
  await connection
    .requestAirdrop(taker.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log);
});

describe("vault-2024", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vault2024 as Program<Vault2024>;

  it("Deposit SOL into Vault!", async () => {
    const tx = await program.methods
      .deposit(seed, new anchor.BN(1 * LAMPORTS_PER_SOL))
      .accounts({
        maker: maker.publicKey,
        taker: taker.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Claim test", async () => {
    const tx = await program.methods
      .claim()
      .accounts({
        taker: taker.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([taker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Deposit SOL into Vault!", async () => {
    const tx = await program.methods
      .deposit(seed, new anchor.BN(1 * LAMPORTS_PER_SOL))
      .accounts({
        maker: maker.publicKey,
        taker: taker.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Cancel, remove SOL from Vault", async () => {
    const tx = await program.methods
      .cancel()
      .accounts({
        maker: maker.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log);
  });
});
