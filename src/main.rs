use std::time::Instant;
use std::{convert::TryFrom, sync::mpsc};

use std::thread;

use solana_sdk::{
    pubkey::Pubkey,
    signature::write_keypair_file,
    signer::{keypair::Keypair, Signer},
};

fn main() {
    let string_to_find = std::env::args().nth(1).expect("No pattern given");

    let thread_count: usize = match std::env::args().nth(2) {
        Some(tc) => tc.parse().expect("Thread count must be a number"),
        None => 10,
    };

    let mut tries: u64 = 1;

    let char_length = string_to_find.len();

    let (tx, rx) = mpsc::channel();

    let start = Instant::now();

    println!(
        "Searching with {} threads for PDA that starts with '{}'",
        thread_count, string_to_find
    );

    for _ in 0..thread_count {
        let thread_tx = tx.clone();
        let to_find = string_to_find.clone();

        thread::spawn(move || loop {
            thread_tx.send(1).unwrap();

            let keypair = Keypair::new();
            let create_key = keypair.pubkey();
            let program_id =
                Pubkey::try_from("SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu").unwrap();

            let (ms_pda, _) = Pubkey::find_program_address(
                &[b"squad", create_key.as_ref(), b"multisig"],
                &program_id,
            );

            let authority_index: u32 = 1;

            let (pda, _) = Pubkey::find_program_address(
                &[
                    b"squad",
                    ms_pda.as_ref(),
                    &authority_index.to_le_bytes(),
                    b"authority",
                ],
                &program_id,
            );

            let pda_string = pda.to_string();

            let to_match = &pda_string[..char_length].to_ascii_lowercase();

            if to_match.eq(&to_find) {
                println!(
                    "Found match: Create Key {} results in {} on Squads Multisig",
                    create_key.to_string(),
                    pda_string
                );

                let filename = "Squads-".to_string() + &pda_string + ".json";

                match write_keypair_file(&keypair, &filename) {
                    Ok(file) => file,
                    Err(error) => panic!("Problem opening the file: {:?}", error),
                };

                println!("Written to file: {}", filename);

                // tell main thread we gucci
                thread_tx.send(0).unwrap();
            }
        });
    }

    loop {
        let msg = rx.recv().unwrap();

        // if one of the threads found a match, terminate
        if msg == 0 {
            println!("Found after {} searches in {:?}", tries, start.elapsed());
            break;
        }

        tries += 1;

        if tries % 10000 == 0 {
            println!("Searched {} keypairs in {:?}", tries, start.elapsed());
        }
    }
}
