import { Connection, Keypair } from "@solana/web3.js";
import { createMint } from "@solana/spl-token"

import wallet from "./wallet/wba-wallet.json"

//Connect our WBA Wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection to devnet SOL tokens
const connection = new Connection("https://api.devnet.solana.com", {commitment: "confirmed"});

(async () => {
  
  const mint = await createMint(
    connection,
    keypair, // payer
    keypair.publicKey, // mint authority
    keypair.publicKey, // freeze authority
    6, // decimals
  );

  console.log(`The unique identifier of the token is: ${mint.toBase58()}`); 

})();