import * as anchor from "@coral-xyz/anchor";
import { Program, BN, web3 } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { randomBytes } from "crypto";
import { createMint, mintTo, Account, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token"

const commitment: web3.Commitment = "confirmed";

const confirmTx = async (signature: string) => {
  console.log("Confirming transaction: ", signature);
  const latestBlockhash = await anchor
  .getProvider()
  .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ... latestBlockhash
    },
    commitment
  );
};

const confirmTxs = async (signatures: string[]) => {
  await Promise.all(signatures.map(confirmTx));
};

describe("amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Amm as Program<Amm>;
  const connection = anchor.getProvider().connection;

  const [initializer, user] = [
    web3.Keypair.generate(),
    web3.Keypair.generate(),
  ];

  const seed = new BN(randomBytes(8));

  const [auth] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("auth")],
    program.programId
  );

  const [config] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config"), seed.toBuffer().reverse()],
    program.programId
  );

  let mint_x: web3.PublicKey;
  let mint_y: web3.PublicKey;
  let [mint_lp] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("lp"), config.toBuffer()],
    program.programId
  );

  let initializer_x_ata: Account;
  let initializer_y_ata: Account;
  let initializer_lp_ata: web3.PublicKey;
  let user_x_ata: Account;
  let user_y_ata: Account;
  let user_lp_ata: web3.PublicKey;
  let vault_x_ata: web3.PublicKey;
  let vault_y_ata: web3.PublicKey;
  let vault_lp_ata: web3.PublicKey;

  it("Airdrop", async () => {
    await Promise.all(
      [initializer, user].map(async (k) => {
        return await anchor
            .getProvider()
            .connection.requestAirdrop(
              k.publicKey,
              100 * anchor.web3.LAMPORTS_PER_SOL
            );
          })
        ).then(confirmTxs);
  });
  
  it("Create mint token", async () => {
    mint_x = await createMint(connection, user, user.publicKey, null, 6);
    mint_y = await createMint(connection, user, user.publicKey, null, 6);
  });

  it("Create ata", async () => {

    initializer_x_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      initializer,
      mint_x,
      initializer.publicKey
    );
    initializer_y_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      initializer,
      mint_y,
      initializer.publicKey
    );
    initializer_lp_ata = await getAssociatedTokenAddressSync(
      mint_lp,
      initializer.publicKey
    );
    user_x_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint_x,
      user.publicKey
    );
    user_y_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint_y,
      user.publicKey
    );

    user_lp_ata = getAssociatedTokenAddressSync(mint_lp, user.publicKey);
    vault_x_ata = getAssociatedTokenAddressSync(mint_x, auth, true);
    vault_y_ata = getAssociatedTokenAddressSync(mint_y, auth, true);
    vault_lp_ata = getAssociatedTokenAddressSync(mint_lp, auth, true);

    await mintTo(
      connection,
      user,
      mint_x,
      initializer_x_ata.address,
      user.publicKey,
      21e8
    ).then(confirmTx);
    await mintTo(
      connection,
      user,
      mint_y,
      initializer_y_ata.address,
      user.publicKey,
      21e8
    ).then(confirmTx);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(seed, 300, initializer.publicKey)
    .accounts({
      initializer: initializer.publicKey,
      mintX: mint_x,
      mintY: mint_y,
      mintLp: mint_lp,
      vaultX: vault_x_ata,
      vaultY: vault_y_ata,
      auth,
      config,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([initializer])
    .rpc();
    confirmTx(tx);
  });

  it("Deposit", async () => {
    const tx = await program.methods
    .deposit(
      new BN(20 * 10 ** 6),
      new BN(20 * 10 ** 6),
      new BN(20 * 10 ** 6),
      new BN(Math.floor(new Date().getTime() / 1000) + 600),
    )
    .accounts({
      user: initializer.publicKey,
      mintX: mint_x,
      mintY: mint_y,
      mintLp: mint_lp,
      vaultX: vault_x_ata,
      vaultY: vault_y_ata,
      userX: initializer_x_ata.address,
      userY: initializer_y_ata.address,
      userLp: initializer_lp_ata,
      auth,
      config,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId
    })
    .signers([initializer])
    .rpc();
    confirmTx(tx);
  });

  it("Swap X for Y", async () => {
    const tx = await program.methods
    .swap(
      true,
      new BN(5 * 10 ** 6),
      new BN(5 * 10 ** 6),
      new BN(Math.floor(new Date().getTime() / 1000) + 600))
    .accounts({
      user: initializer.publicKey,
      mintX: mint_x,
      mintY: mint_y,
      mintLp: mint_lp,
      vaultX: vault_x_ata,
      vaultY: vault_y_ata,
      userX: initializer_x_ata.address,
      userY: initializer_y_ata.address,
      userLp: initializer_lp_ata,
      auth,
      config,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId
    })
    .signers([initializer])
    .rpc({skipPreflight:true});
    confirmTx(tx);
  });
});
