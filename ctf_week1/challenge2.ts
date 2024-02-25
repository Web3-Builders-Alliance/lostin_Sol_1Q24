/*
send #@% amount of token to the Vault.
The document seems to be bugged… i wonder if anyone have already sent the right amount of token, maybe someone that tested it…
Leo Wallet: AaPVSNu9qRC8BG2SzUr1JS9ZZJS35NHNMSArhV3JVDsz
*/

import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { Week1, IDL } from "./programs/week1";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

import wallet from "./wallet/wba-wallet.json"

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Create our program
const program = new Program<Week1>(IDL, "ctf1VWeMtgxa24zZevsXqDg6xvcMVy4FbP3cxLCpGha" as Address, provider);

// Use the PDA for our CTF-Week1 profile
const profilePda = PublicKey.findProgramAddressSync([Buffer.from("profile"), keypair.publicKey.toBuffer()], program.programId)[0];

// Paste here the mint address for challenge2 token
const mint = new PublicKey("9HVjN6BidXf9kX264gYifhGppvAJ3LmLKdH2rt79hapm");

// Create the PDA for the Challenge2 Vault
const vault = PublicKey.findProgramAddressSync([Buffer.from("vault2"), keypair.publicKey.toBuffer(), mint.toBuffer()], program.programId)[0];

(async () => {
    try {

        // NB if you get TokenAccountNotFoundError, wait a few seconds and try again!

        const ownerAta = await getOrCreateAssociatedTokenAccount(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            keypair.publicKey // owner  // TODO: Is this owner of the mint account or owner of the ATA?
        );

        // // Mint some tokens!
        const mintTx = await mintTo(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            (await ownerAta).address, // dest: PublicKey
            keypair, // authority: Signer
            100000 // amount: BN
        );
        
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${mintTx}?cluster=devnet`);
        
        // Complete the Challenge!
        const completeTx = await program.methods.completeChallenge2(new BN(255))
        .accounts({
            owner: keypair.publicKey,
            ata: (await ownerAta).address,
            profile: profilePda,
            vault: vault,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
        })
        .signers([
            keypair
        ]).rpc();

        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${completeTx}?cluster=devnet`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();