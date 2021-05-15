pub const USAGE_INFO: &str = "
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signing of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff generateRandom
        etheroff [--keyfile=<path>]
        etheroff signTransaction <to> <value> <nonce> [--gasLimit=<uint>] [--gasPrice=<uint>] [--data=<bytes>] [--chainId=<uint>] [--keyfile=<path>] [--logLevel=<string>]

Commands:

        version                ❍ Show version info.
        generateRandom         ❍ Generate random ETH private key & address.
        signTransaction        ❍ Sign an ethereum transaction.
        <to>                   ❍ Ethereum address to send the transaction to.
        <nonce>                ❍ The nonce to be used for the ethereum transaction.
        <value>                ❍ How much ether to send (in Wei!)
        <no-command>           ❍ Run the interactive Q&A version of this tool.

Options:

        --help                 ❍ Show this message.
        --keyfile=<path>       ❍ Path to GPG-encrypted ETH private key in hex format. [default: ./pk.gpg]
        --chainId=<uint>       ❍ The integer used to identify which chain to sign a transaction for.  [default: 1]
        --data=<bytes>         ❍ Any calldata required for the transaction, in hex format. [default: 0x]
        --gasLimit=<uint>      ❍ The gas limit for the ETH transaction. [default: 21000]
        --gasPrice=<uint>      ❍ The gas price for the ETH transaction. [default: 20000000000]
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`,
                                 `trace` or `error`. [default: none]

";
