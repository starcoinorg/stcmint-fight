# stcmint-fight
Caculate The result of [Starcoin testnet mining competition round 1](http://news.starcoin.org/en/2020/starcoin_testnet_mining_competition_r1/)

#### Top n
Caculte the top n addresses during the competition time
#### Select the lucky addresses
Every address that participates in the mining competiton may become a lucky address

* How lucky addresses are calculated
    1. Lucky address pool: Addresses that have successfully mined blocks during the contest period will be entered into the address pool, and the weight of the lucky address will be **1+ Log2 block_count**, and will be sorted by block_count, where block_count is the total number of blocks mined by the address.
    2. Luck address select
        * Defined lucky address pool size as N, N is the size after weight correction.
        * Take nonces as the seeds from some special block headers.
        * set the lucky address index = seed mod N. The lucy address is corresponding to the index related value in the address pool.

Enjoy your hacking.
