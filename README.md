## Brands Scheme

A digital cash scheme proposed by Stefan Brands in his paper [Untraceable Off-line Cash in Wallets with Observers](https://dl.acm.org/doi/10.5555/188105.188172) (Crypto '93, Stefan Brands (1993)).

It describes an electronic cash system that provides two key properties:
1. Offline anonymous payment - payment can take place in private manner without accessing internet 
2. Traceable double spender - the identity of spender will be revealed eventually if using same "digital cash" in other payments.

**Note:** The code in this repository is not implemented for being used in production environment. Please take the risks into your considerations before you use it. Also, it is recommended to look at some security analysis after the paper was published. 

### Payment Model

The payment system involves parties:
- `Issuer` who issues coins to `Spender`, and traces double spenders upon receiving coins from `Receiver`;
- `Spender` who spends the coins in payment;
- `Receiver` who receives coins and deposits them back to `Issuer`

### Usage

The library provides full implementations for the abstract structs `Issuer` and `Spender`, as well as relevant structs and function for verifying a coin. Here is to describe the account setup and payment processes in a program flow.

First, the parameters of brands scheme has to be public to the parties, which is encapsulated in the struct `brands::Params`.

To instantiate the structs for parties `Issuer` and `Spender`,

```rust
let issuer = Issuer::new(params.clone());
let mut spender = Spender::new(params.clone());
```

In Account opening, `Issuer` receives the spender's identity. After authentication (not provided in this library), `Issuer` responses to the spender with registration ID,

```rust
spender.set_registration_id(issuer.register(&spender.i));
```

In coin withdrawal, `Issuer` generates parameters for the process. Then, `Spender` creates a challenge and uses the associated response to make a coin. At this point, the `Issuer` has no knowledge of this coin and its association with `Spender`.

```rust
// Withdraw a coin
// 1. Issuer setup parameters
let (
    withdrawal_params,
    withdrawal_response_params
) = issuer.setup_withdrawal_params(&spender.i);
// 2. Spender creates a challenge for issuer
let (
    withdrawal,
    withdrawal_challenge
) = spender.withdraw(withdrawal_params);
// 3. Issuer responses
let withdrawal_response = issuer.withdrawal_response(withdrawal_response_params, &withdrawal_challenge);
// 4. (Optional) Spender verifies the response
assert!(spender.verify_withdrawal_response(
    &issuer.h,
    &withdrawal,
    &withdrawal_challenge,
    &withdrawal_response
));
// 5. Spender makes a coin from the response
let coin = spender.make_coin(&withdrawal, withdrawal_response);
```

`Spender` can spend the coin to the `Receiver` in an interactive process which requires `Spender` to "prove" the knowledge of the coin.

```rust
// Spend a coin
// 1. Receiver verifies the coin
assert!(coin.verify(&issuer.h, &params));
// 2. Receiver challenges the spender
let challenge = CoinChallenge::new("shopA-payment-item-1718193570".as_bytes(), &coin);
// 3. Spender responds
let partial_coin = PartialCoin::from(withdrawal);
let spent_coin = spender.spend(coin, partial_coin, &challenge);
// 4. Receiver verifies the spent coin
assert!(spent_coin.verify(&challenge, &params));
```

Finally, `Receiver` sends back the spent coin and the coin challenge to `Issuer`. `Issuer` can reveal the identity of the spender.

```rust
// Suppose spent_coin_2 is the double spent coin.
let i = spent_coin.reveal_identity(&spent_coin_2, &params);
```