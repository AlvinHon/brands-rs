use brands::{CoinChallenge, Issuer, Params, PartialCoin, Spender};
use diffie_hellman_groups::MODPGroup5;

#[test]
fn test_double_spent_coin_lifecycle() {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());

    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());
    println!("Spender Identity: {}", spender.i);

    // Account Setup
    spender.set_registration_id(issuer.register(&spender.i));

    // Withdraw a coin
    // 1. Issuer setup parameters
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    // 2. Spender creates a challenge for issuer
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params);
    // 3. Issuer responses
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params, &withdrawal_challenge);
    // 4. (Optional) Spender verifies the response
    assert!(spender.verify_withdrawal_response(
        &issuer.h,
        &withdrawal,
        &withdrawal_challenge,
        &withdrawal_response
    ));
    // 5. Spender makes a coin from the response
    let coin = spender.make_coin(&withdrawal, withdrawal_response);

    // Spend a coin
    // 1. Receiver verifies the coin
    assert!(coin.verify(&issuer.h, &params));
    // 2. Receiver challenges the spender
    let challenge = CoinChallenge::new("shopA-payment-item-1718193570".as_bytes(), &coin);
    // 3. Spender responds
    let partial_coin = PartialCoin::from(withdrawal);
    let spent_coin = spender.spend(coin.clone(), partial_coin.clone(), &challenge);
    // 4. Receiver verifies the spent coin
    assert!(spent_coin.verify(&challenge, &params));

    // Deposit the coin to Issuer
    // .. Receiver sends spent_coin to issuer for checking if it is double spent ..

    // !! Spender double spending !!
    let challenge_2 = CoinChallenge::new("shopB-payment-item-1718193571".as_bytes(), &coin);
    let spent_coin_2 = spender.spend(coin.clone(), partial_coin, &challenge_2);
    assert!(spent_coin_2.verify(&challenge_2, &params));

    // Deposit the coin to Issuer
    // .. Receiver sends spent_coin to issuer for checking if it is double spent ..

    // !! Issuer found same coin has been spent !!

    // Recover the identity of the spender
    let i = spent_coin.reveal_identity(&spent_coin_2, &params);
    println!("Double spender is: {}", i);
    assert_eq!(i, spender.i);
}
