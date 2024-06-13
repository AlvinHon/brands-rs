use brands::{hash_to_number, Issuer, Params, Spender};


#[test]
fn test_double_spent_coin_lifecycle() {
    let params = Params::new(1024 * 1024);
    println!(
        "p={} q={} g={} g1={} g2={}",
        params.p, params.q, params.g, params.g1, params.g2
    );

    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());
    println!("Spender Identity: {}", spender.i);

    spender.set_registered(issuer.register(&spender.i));

    // Account Setup
    let withdrawal_params = issuer.withdrawal_params(&spender.i);
    let withdrawal = spender.withdraw(&withdrawal_params.a, &withdrawal_params.b);
    let response = issuer.withdrawal_response(&withdrawal_params, &withdrawal.challenge);
    assert!(spender.verify_withdrawal_response(&issuer.h, &response, &withdrawal));

    // Make a coin
    let coin = spender.make_coin(response, withdrawal.clone());

    // Spend a coin
    // 1. Receiver verifies the coin
    assert!(coin.verify(&issuer.h, &params));
    // 2. Receiver challenges the spender
    let challenge = hash_to_number(
        "shopA-payment-item-1718193570".as_bytes(),
        &[coin.c1.to_bytes_le(), coin.c2.to_bytes_le()],
    );
    // 3. Spender responds
    let spent_coin = spender.spent_coin(&coin, &challenge, &withdrawal);
    // 4. Receiver verifies the spent coin
    assert!(spent_coin.verify(&challenge, &params));

    // Deposit the coin to Issuer
    // .. Receiver sends spent_coin to issuer for checking if it is double spent ..

    // !! Spender double spending !!
    let challenge_2 = hash_to_number(
        "shopB-payment-item-1718193571".as_bytes(),
        &[coin.c1.to_bytes_le(), coin.c2.to_bytes_le()],
    );
    let spent_coin_2 = spender.spent_coin(&coin, &challenge_2, &withdrawal);
    assert!(spent_coin_2.verify(&challenge_2, &params));

    // Deposit the coin to Issuer
    // .. Receiver sends spent_coin to issuer for checking if it is double spent ..

    // !! Issuer found same coin has been spent !!

    // Recover the identity of the spender
    let i = issuer.double_spender_identity(&spent_coin, &spent_coin_2);
    println!("Double spender is: {}", i);
    assert_eq!(i, spender.i);
}