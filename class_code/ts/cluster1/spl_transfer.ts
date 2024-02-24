import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wallet/dev-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed"; // processed | confirmed | finalized
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address, previous exercise
const mint = new PublicKey("75NJm84GS4PKQVwxotjZfEjnxmBP3FTZgaAxxSfcEGh1");

// Recipient address, can be random address
const to = new PublicKey("CnfAC8WoLLhVcB35yW9UZzWFNDaFpNk3nfYRM4e64ipZ");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            keypair.publicKey // owner: PublicKey
        );

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            to // owner: PublicKey
        );
        
        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(
            connection, // Connection
            keypair, // payer: Signer
            fromTokenAccount.address, // source: PublicKey
            toTokenAccount.address, // dest: PublicKey
            keypair, // owner: Signer
            1000n // amount: number
        );
        console.log(`Your transaction txid: ${tx}`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();