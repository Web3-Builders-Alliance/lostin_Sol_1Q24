import wallet from "./wallet/dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    getAccountMetasAndSigners
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";

// Define our Mint address
const mint = publicKey("75NJm84GS4PKQVwxotjZfEjnxmBP3FTZgaAxxSfcEGh1")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));
  
(async () => {
    try {

        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer,
            payer: signer
        }

        let data: DataV2Args = {
            name: "Oooooops",
            symbol: "OOPS",
            uri: "https://en.wikipedia.org/wiki/Oops!_(film)#/media/File:Oops_poster.jpg",
            creators: [
                {
                    address: keypair.publicKey,
                    verified: true, // guarantees token was signed by that creator
                    share: 100
                }
            ],
            sellerFeeBasisPoints: 0,
            collection: null,
            uses: null, // ?
        }


        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null
        }

        // let tx = updateMetadataAccountV2(
        //     umi,
        //     {
        //         ...accounts,
        //         ...args
        //     }
        // )

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi).then(r => r.signature.toString());
        console.log(result);
        console.log(`Transaction: ${tx}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();