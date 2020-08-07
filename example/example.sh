#!/bin/bash
../target/release/etheroff \
signTransaction \
0xfEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC \
1 \
1337 \
--gasLimit=30000 \
--gasPrice=60000000000 \
--keyfile=./example-private-key.gpg \
--chainId=3 \
--logLevel=info
