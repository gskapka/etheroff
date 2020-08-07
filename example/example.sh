#!/bin/bash
../target/release/etheroff \
signTransaction \
--keyfile=./example-private-key.gpg \
--chainId=3 \
--logLevel=info
