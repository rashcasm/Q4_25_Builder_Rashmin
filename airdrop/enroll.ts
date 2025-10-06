import {
  address,
  addSignersToTransactionMessage,
  appendTransactionMessageInstructions,
  assertIsTransactionWithinSizeLimit,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  devnet,
  getSignatureFromTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  getProgramDerivedAddress,
  generateKeyPairSigner,
  getAddressEncoder,
} from '@solana/kit';

import { getInitializeInstruction, getSubmitTsInstruction } from "./clients/js/src/generated/index";

import wallet from './Turbin3-wallet.json';

// import wallet from './testwallet.json';
const MPL_CORE_PROGRAM = address('CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d');
const PROGRAM_ADDRESS = address('TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM');
const SYSTEM_PROGRAM = address('11111111111111111111111111111111');

const COLLECTION = address('5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2');

// We're going to import our keypair from the wallet file
const keypair = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
const rpc = createSolanaRpc(devnet('https://api.devnet.solana.com'));

// Create a devnet connection
const rpcSubscriptions = createSolanaRpcSubscriptions(
  devnet('wss://api.devnet.solana.com')
);
//address
console.log('Your wallet address:', keypair.address);



const addressEncoder = getAddressEncoder();
// Create the PDA for enrollment account
const accountSeeds = [
  Buffer.from('prereqs'),
  addressEncoder.encode(keypair.address),
];
const [account, _bump] = await getProgramDerivedAddress({
  programAddress: PROGRAM_ADDRESS,
  seeds: accountSeeds,
});

//logging
console.log('Account PDA:', account);

const authoritySeeds = [
  Buffer.from('collection'),
  addressEncoder.encode(COLLECTION),
];
const [authority] = await getProgramDerivedAddress({
  programAddress: PROGRAM_ADDRESS,
  seeds: authoritySeeds,
});

console.log('Authority PDA:', authority);

// Generate mint keypair for the NFT
const mintKeyPair = await generateKeyPairSigner();
console.log('Mint address:', mintKeyPair.address);

// Execute the initialize transaction
const initializeIx = getInitializeInstruction({
github: "rashcasm",
user: keypair,
account,
systemProgram: SYSTEM_PROGRAM
});

// Fetch latest blockhash
const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

const transactionMessageInit = pipe(
createTransactionMessage({ version: 0 }),
tx => setTransactionMessageFeePayerSigner(keypair, tx),
tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash,
tx),
tx => appendTransactionMessageInstructions([initializeIx], tx)
);

const signedTxInit = await
signTransactionMessageWithSigners(transactionMessageInit);
assertIsTransactionWithinSizeLimit(signedTxInit);

const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({rpc, rpcSubscriptions });

try {
const result = await sendAndConfirmTransaction(
signedTxInit,
{ commitment: 'confirmed', skipPreflight: false }
);
console.log(result);
const signatureInit = getSignatureFromTransaction(signedTxInit);
console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signatureInit}?cluster=devnet`);
} catch (e) {
console.error(`Oops, something went wrong: ${e}`);
}

const { value: latestBlockhash2 } = await rpc.getLatestBlockhash().send();

const submitIx = getSubmitTsInstruction({
  user: keypair,
  account,
  mint: mintKeyPair,
  collection: COLLECTION,
  authority,
  mplCoreProgram: MPL_CORE_PROGRAM,
  systemProgram: SYSTEM_PROGRAM,
});

// const submitTransactionMessage = pipe(
//   createTransactionMessage({ version: 0 }),
//   (tx) => setTransactionMessageFeePayerSigner(keypair, tx),
//   (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash2, tx),
//   (tx) => appendTransactionMessageInstructions([submitIx], tx),
//   (tx) => addSignersToTransactionMessage([mintKeyPair], tx)
// );

const transactionMessageSubmit = pipe(
createTransactionMessage({ version: 0 }),
tx => setTransactionMessageFeePayerSigner(keypair, tx),
tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
tx => appendTransactionMessageInstructions([submitIx], tx),
tx => addSignersToTransactionMessage([mintKeyPair], tx) // Add mint as additional signer after appending instructions
);
//const signedTxSubmit = await signTransactionMessageWithSigners(submitTransactionMessage);
const signedTxSubmit = await signTransactionMessageWithSigners(transactionMessageSubmit);

assertIsTransactionWithinSizeLimit(signedTxSubmit);

try {
  await sendAndConfirmTransaction(signedTxSubmit, {
    commitment: 'confirmed',
  });
  const signatureSubmit = getSignatureFromTransaction(signedTxSubmit);
console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signatureSubmit}?cluster=devnet`);
} catch (e) {
console.error(`Oops, something went wrong: ${e}`);

}

