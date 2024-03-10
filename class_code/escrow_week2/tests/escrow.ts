import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { LAMPORTS_PER_SOL, PublicKey, Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { MINT_SIZE, ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createInitializeMint2Instruction, createMintToInstruction, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint, TokenAccountNotFoundError} from "@solana/spl-token";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Escrow as Program<Escrow>;
const provider = anchor.getProvider();
const connection = anchor.getProvider().connection;

const confirm = async (signature: string): Promise<string> => {
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature,
    ...block,
  });
  return signature;
};

const log = async (signature: string): Promise<string> => {
  console.log(`Your txs: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
  return signature;
};

describe("escrow", () => { 
  
  const maker = Keypair.generate();
  const taker = Keypair.generate();
  const mintX = Keypair.generate();
  const mintY = Keypair.generate();
  const makerAtaX = getAssociatedTokenAddressSync(
    mintX.publicKey,
    maker.publicKey
  );
  const makerAtaY = getAssociatedTokenAddressSync(
    mintY.publicKey,
    maker.publicKey
  );
  const takerAtaX = getAssociatedTokenAddressSync(
    mintX.publicKey,
    taker.publicKey
  );
  const takerAtaY = getAssociatedTokenAddressSync(
    mintY.publicKey,
    taker.publicKey
  );

  const seed = new anchor.BN(1);
  const [escrow] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toBuffer("le", 8)],
    program.programId
  );

  const vault = getAssociatedTokenAddressSync(
    mintX.publicKey,
    escrow, 
    true
  );

  it("Airdrop", async () => {
    await Promise.all([
      await connection
        .requestAirdrop(maker.publicKey, 3 * LAMPORTS_PER_SOL)
        .then(confirm),
      await connection
        .requestAirdrop(taker.publicKey, 3 * LAMPORTS_PER_SOL)
        .then(confirm),
    ]);
  });

  it("Setup Mint", async () => {
    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();
    tx.instructions = [
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mintX.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID
      }),
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mintY.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID
      }),
      createInitializeMint2Instruction(
        mintX.publicKey,
        9,
        maker.publicKey,
        null
      ),
      createInitializeMint2Instruction(
        mintY.publicKey,
        9,
        taker.publicKey,
        null
      ),
      createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        makerAtaX,
        maker.publicKey,
        mintX.publicKey
      ),
      createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        takerAtaY,
        taker.publicKey,
        mintY.publicKey
      ),
      createMintToInstruction(
        mintX.publicKey,
        makerAtaX,
        maker.publicKey,
        1e9
      ),
      createMintToInstruction(
        mintY.publicKey,
        takerAtaY,
        taker.publicKey,
        1e9
      ),
    ];

    await provider.sendAndConfirm(tx, [mintX, mintY, maker, taker]).then(log);
  });

    it("Make new", async () => {
      await program.methods
      .makeEscrow(seed, new anchor.BN(100e6))
      .accounts({
        maker: maker.publicKey,
        escrow,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        vault,
        makerAtaX: makerAtaX,
        makerAtaY: makerAtaY,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([maker])
      .rpc()
      .then(log);
    });

    it.skip("Refund", async() => {
      await program.methods
      .refundEscrow()
      .accounts({
        maker: maker.publicKey,
        escrow,
        vault,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        makerAtaX: makerAtaX,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then(log);
    });

    it("Take", async() => {
      await program.methods
      .takeEscrow()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        escrow,
        vault,
        mintX: mintX.publicKey,
        mintY: mintY.publicKey,
        takerAtaX: takerAtaX,
        takerAtaY: takerAtaY,
        makerAtaY: makerAtaY,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([taker])
      .rpc()
      .then(log);
    });
});