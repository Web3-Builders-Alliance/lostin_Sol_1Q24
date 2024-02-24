import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "./wallet/dev-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed"; // processed | confirmed | finalized
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n; // n denotes a bigint

// Mint address - Hardcoded from previous exercise
const mint = new PublicKey("75NJm84GS4PKQVwxotjZfEjnxmBP3FTZgaAxxSfcEGh1");

(async () => {
    try {
        // Create an ATA associated token account for the mint
        const ata = await getOrCreateAssociatedTokenAccount(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            keypair.publicKey // owner: PublicKey
        );
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint to ATA
        const mintTx = await 
        mintTo(
            connection, // Connection
            keypair, // payer: Signer
            mint, // mint: PublicKey
            ata.address, // dest: PublicKey
            keypair, // authority: Signer
            1n * token_decimals // amount: number
        );
        console.log(`Your mint txid: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
