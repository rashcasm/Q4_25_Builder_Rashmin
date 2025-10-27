
#[cfg(test)]
mod tests {
    use solana_sdk::{self, message::Message};
    use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
//        hash::hash,
        pubkey::Pubkey,
        transaction::Transaction,
    };
    use std::str::FromStr;
    use solana_system_interface::{program as system_program, instruction::transfer};
    use solana_sdk::instruction::{Instruction,AccountMeta};


    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}\n", kp.pubkey());
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g.
        [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn claim_airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json")
            .expect("Couldn't find wallet file");
        // We'll establish a connection to Solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL);
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }


    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        
        // let sig_hashed = hash(sig.as_ref());
        // Verify the signature using the public key
        // match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        //     true => println!("Signature verified"),
        //     false => println!("Verification failed"),
        // }
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        //define destination address
        let to_pubkey = Pubkey::from_str("GsJYonU5Kz4MJBHZ5UFx9oyStBpXXswnZcFUorktj2yZ").unwrap();
        //connect to solana devnet
        let rpc_client = RpcClient::new(RPC_URL);
        // fetch recent blockchash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        // 0.1 from dev to turbin3
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        //send and print tx
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            println!(
            "Success! Check out your TX here:
            https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet(){
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        //define destination address
        let to_pubkey = Pubkey::from_str("GsJYonU5Kz4MJBHZ5UFx9oyStBpXXswnZcFUorktj2yZ").unwrap();
        //connect to solana devnet
        let rpc_client = RpcClient::new(RPC_URL);
        // fetch recent blockchash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // mock transaction
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // estimate fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // final transaction 
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // send and verify
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");
        println!(
            "Success! Entire balance transferred:
            https://explorer.solana.com/tx/{}/?cluster=devnet", signature
        );
    }


    #[test]
    fn submit_rs(){
        let rpc_client = RpcClient::new(RPC_URL);
        // load signer pair
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        // Generate a signature from the keypair
        let pubkey = signer.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = signer.sign_message(message_bytes);
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        // Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();
        let authority = Pubkey::from_str("5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k")
        .expect("Invalid authority PDA");
        // creating PDA
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds,&turbin3_prereq_program);
        // prepare inx data
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        //define accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false), // PDA account
            AccountMeta::new(mint.pubkey(), true), // mint keypair
            AccountMeta::new(collection, false), // collection
            AccountMeta::new_readonly(authority, false), // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];
        // recent blockhash
        let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        // create instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
