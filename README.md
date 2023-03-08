# Gratie Solana


## ROADMAP

* [x] Create reward tokens with erc-20 tokens based on the evaluation of the company
* [ ] Add metadata to these reward tokens
* [ ] Create a rewards bucket (account) for every user of the company
* [ ] Unsafe: Create a link that allows the user to withdraw from the bucket to his account, this link will be sent to the user via email, maybe do some verification using merkle trees?



## Notes

* changing structure of accounts in the src/state folder might break the tests because existing accounts on the network will have a different structure
* it would be great to have metaplex on the localnet

### Metaplex solana test validator
* clone this repo: [metpaplex](https://github.com/metaplex-foundation/js)
* run yarn && yarn amman:start
* go to [amman](https://amman-explorer.metaplex.com/) in your browser;