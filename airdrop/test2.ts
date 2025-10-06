import { address, createSolanaRpc, devnet } from "@solana/kit";

async function main() {
  // Create RPC connection
  const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));

  // Define the public key
  const publicKey = address("6mE2rmCYXpmXGmoDt2QoCHxJBwKHdVJLBBpXTSSQighV");

  // Get the balance
  const balanceObj = await rpc.getBalance(publicKey).send();

  // Extract lamports
  const lamportsBalance = balanceObj.value;

  // Convert lamports to SOL
  const solBalance = Number(lamportsBalance) / 1_000_000_000;

  console.log(`Account: ${publicKey}`); // Use the address object as string
  console.log(`Balance: ${lamportsBalance} lamports`);
  console.log(`Balance: ${solBalance} SOL`);
}

main().catch(console.error);
