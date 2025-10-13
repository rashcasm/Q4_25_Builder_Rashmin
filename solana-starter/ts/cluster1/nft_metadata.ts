import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/E33EY6j3maReWSbMVAZEU7QBoC4HnuQJ71tf6N7Ch7Tn"
        const metadata = {
            name: "rashcasmRug",
            symbol: "minrug",
            description: "Jeff's rug, now an NFT",
            image,
            attributes: [
                {trait_type: 'creator', value: 'rashcasm'},
                {trait_type: 'character', value: 'jeff'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image,
                    },
                ]
            },
            creators: ["Rashmin"]
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// https://gateway.irys.xyz/CBJx6cyFktzRAq1pLFKYth31QeVxitF4pN812WFLDHEm