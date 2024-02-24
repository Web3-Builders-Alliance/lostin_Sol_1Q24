import wallet from "./wallet/dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://arweave.net/YFvbHhzWEeLUV_tnXv90Yn4tbn19f8ehVd8U3YHDuEA"

        const metadata = {
            name: "RugRug",
            symbol: "RUG",
            description: "A generative rug",
            image: image,
            attributes: [
                {trait_type: 'Color', value: 'Blue'},
                {trait_type: 'Style', value: 'Persian'},
                {trait_type: 'Age', value: 'Old'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            }
        };

        // const myUri = await umi.uploader.uploadJson([metadata]);
        const metadataUri = await bundlrUploader.uploadJson(metadata);
        console.log("Your image URI: ", metadataUri);

    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();