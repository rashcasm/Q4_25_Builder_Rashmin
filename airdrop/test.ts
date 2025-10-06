import bs58 from 'bs58';
import promptSync from 'prompt-sync';

const prompt = promptSync();

const walletBytes: number[] = [
  72,240,58,243,111,199,2,252,240,102,123,153,50,168,111,49,175,144,43,23,
  116,235,238,149,234,145,104,252,39,133,192,183,85,158,185,192,215,99,246,41,66,169,4,148,69,129,183,84,138,111,206,69,66,140,195,114,138,155,70,37,22,154,252,96
];

const walletUint8Array = new Uint8Array(walletBytes);

const base58Key = bs58.encode(walletUint8Array);

console.log("Base58 Encoded Key (Phantom):", base58Key);

const userInput = prompt("Enter a Base58 private key to decode (or press enter to skip): ");
if (userInput) {
  const decodedBytes = bs58.decode(userInput);
  console.log("Decoded Byte Array:", Array.from(decodedBytes));
}
