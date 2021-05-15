quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ General program error: {}", err)
        }
        IoError(err: std::io::Error) {
            from()
            display("✘ I/O error: {}", err)
        }
        Secp265k1Error(err: secp256k1::Error) {
            from()
            display("✘ Secp256k1 error: {}", err)
        }
        UTF8Error(err: std::str::Utf8Error) {
            from()
            display("✘ UTF8 error: {}", err)
        }
        FromDecimalStringError(err: ethereum_types::FromDecStrErr) {
            from()
            display("✘ From decimal string error: {}", err)
        }
        TerminalLoggerError(err: simplelog::TermLogError) {
            from()
            display("✘ Terminal-logger error: {}", err)
        }
        DocoptError(err: docopt::Error) {
            from()
            display("✘ Docopt error: {}", err)
        }
        NoneError(err: &'static str) {
            display("✘ None Error!\n✘ {}", err)
        }
    }
}
