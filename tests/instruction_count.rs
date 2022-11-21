use {
    dedup_bpf::{dedup::*, id},
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

#[tokio::test]
async fn test_dedup_76() {
    // test naive and hashset dedup
    let mut pc = ProgramTest::new("dedup_bpf", id(), processor!(process_instruction));
    pc.set_compute_max_units(100_000);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    // 76 is the point where hashset dedup beats naive dedup
    let mut transaction =
        Transaction::new_with_payer(&[instruction_dedup_vector(76)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction =
        Transaction::new_with_payer(&[instruction_dedup_hashset(76)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_dedup_32() {
    // test naive and hashset dedup
    let mut pc = ProgramTest::new("dedup_bpf", id(), processor!(process_instruction));
    pc.set_compute_max_units(100_000);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    // 76 is the point where hashset dedup beats naive dedup
    let mut transaction =
        Transaction::new_with_payer(&[instruction_dedup_vector(32)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction =
        Transaction::new_with_payer(&[instruction_dedup_hashset(32)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
