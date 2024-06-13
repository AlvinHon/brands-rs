## Brands Scheme

A digital cash scheme proposed by Stefan Brands in his paper [Untraceable Off-line Cash in Wallets with Observers](https://dl.acm.org/doi/10.5555/188105.188172) (Crypto '93, Stefan Brands (1993)).

It describes an electronic cash system that provides two key properties:
1. Offline anonymous payment - payment can take place in private manner without accessing internet 
2. Traceable double spender - the identity of spender will be revealed eventually if using same "digital cash" in other payments.

**Note:** The code in this repository is not implemented for being used in production environment. Please take the risks into your considerations before you use it. Also, it is recommended to look at some security analysis on the scheme after the paper was published. 

### Payment Model

The payment system involves:
- `Isser` who issues coins to `Spender`, and traces double spenders upon receiving coin from `Receiver`;
- `Spender` who spends the coins in payment;
- `Receiver` who deposits the coins back to `Issuer`

### Example

TODO
