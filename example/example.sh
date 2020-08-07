#!/bin/bash
../target/release/etheroff \
signTransaction \
0xfEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC \
--keyfile=./example-private-key.gpg \
--chainId=3 \
--logLevel=info
