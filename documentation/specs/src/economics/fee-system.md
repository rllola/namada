## Fee system

In order to be accepted by the Namada ledger, transactions must pay fees. Transaction fees serve two purposes: first, the efficient allocation of block space given permissionless transaction submission and varying demand, and second, incentive-compatibility to encourage block producers to add transactions to the blocks which they create and publish.

Namada transaction fees can be paid in any fungible token which is a member of a whitelist controlled by Namada governance. Governance also sets minimum fee rates (which can be periodically updated so that they are usually sufficient) which transactions must pay in order to be accepted (but they can always pay more to encourage the proposer to prioritise them). When using the shielded pool, transactions can also unshield tokens in order to pay the required fees.

The token whitelist consists of a list of $(T, GP_{min})$ pairs, where $T$ is a token identifier and $GP_{min}$ is the minimum price per unit gas which must be paid by a transaction paying fees using that asset. This whitelist can be updated with a standard governance proposal. All fees collected are paid directly to the block proposer (incentive-compatible, so that side payments are no more profitable).