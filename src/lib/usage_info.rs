pub const USAGE_INFO: &str = "
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signging of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff signTransaction [--logLevel=<string>]

Commands:

        version                ❍ Show version info.
        signTransaction        ❍ Generate a random ethereum address.

Options:

        --help                 ❍ Show this message.
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`,
                                 `trace` or `error`. [default: none]

";
