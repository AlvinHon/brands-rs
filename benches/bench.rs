use brands::{CoinChallenge, Issuer, Params, PartialCoin, Spender};
use criterion::{criterion_group, criterion_main, Criterion};
use diffie_hellman_groups::MODPGroup5;

criterion_main!(digital_cash);

criterion_group! {
    name = digital_cash;
    config = Criterion::default();
    targets = bench_register,
            bench_setup_withdrawal_params,
            bench_withdraw,
            bench_withdrawal_response,
            bench_verify_withdrawal_response,
            bench_make_coin,
            bench_verify,
            bench_spend,
            bench_reveal_identity
}

fn bench_register(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let spender = Spender::new(params.clone());

    c.bench_function("register", |b| b.iter(|| issuer.register(&spender.i)));
}

fn bench_setup_withdrawal_params(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));

    c.bench_function("setup_withdrawal_params", |b| {
        b.iter(|| issuer.setup_withdrawal_params(&spender.i))
    });
}

fn bench_withdraw(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, _) = issuer.setup_withdrawal_params(&spender.i);

    c.bench_function("withdraw", |b| {
        b.iter(|| spender.withdraw(withdrawal_params.clone()))
    });
}

fn bench_withdrawal_response(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (_, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());

    c.bench_function("withdrawal_response", |b| {
        b.iter(|| {
            issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge)
        })
    });
}

fn bench_verify_withdrawal_response(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge);

    c.bench_function("verify_withdrawal_response", |b| {
        b.iter(|| {
            spender.verify_withdrawal_response(
                &issuer.h,
                &withdrawal,
                &withdrawal_challenge,
                &withdrawal_response,
            )
        })
    });
}

fn bench_make_coin(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge);

    c.bench_function("make_coin", |b| {
        b.iter(|| spender.make_coin(&withdrawal, withdrawal_response.clone()))
    });
}

fn bench_verify(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge);
    let coin = spender.make_coin(&withdrawal, withdrawal_response.clone());

    c.bench_function("verify", |b| b.iter(|| coin.verify(&issuer.h, &params)));
}

fn bench_spend(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge);
    let coin = spender.make_coin(&withdrawal, withdrawal_response.clone());
    let challenge = CoinChallenge::new("shopA-payment-item-1718193570".as_bytes(), &coin);
    let partial_coin = PartialCoin::from(withdrawal);

    c.bench_function("spend", |b| {
        b.iter(|| spender.spend(coin.clone(), partial_coin.clone(), &challenge))
    });
}

fn bench_reveal_identity(c: &mut Criterion) {
    let params = Params::from_dh_group::<MODPGroup5>("brandskey".to_string());
    let issuer = Issuer::new(params.clone());
    let mut spender = Spender::new(params.clone());

    spender.set_registration_id(issuer.register(&spender.i));
    let (withdrawal_params, withdrawal_response_params) =
        issuer.setup_withdrawal_params(&spender.i);
    let (withdrawal, withdrawal_challenge) = spender.withdraw(withdrawal_params.clone());
    let withdrawal_response =
        issuer.withdrawal_response(withdrawal_response_params.clone(), &withdrawal_challenge);
    let coin = spender.make_coin(&withdrawal, withdrawal_response.clone());
    let challenge = CoinChallenge::new("shopA-payment-item-1718193570".as_bytes(), &coin);
    let partial_coin = PartialCoin::from(withdrawal);
    let spent_coin = spender.spend(coin.clone(), partial_coin.clone(), &challenge);

    let challenge_2 = CoinChallenge::new("shopB-payment-item-1718193571".as_bytes(), &coin);
    let spent_coin_2 = spender.spend(coin.clone(), partial_coin, &challenge_2);

    c.bench_function("reveal_identity", |b| {
        b.iter(|| spent_coin.reveal_identity(&spent_coin_2, &params))
    });
}
