use crate::{
    lib::{
        types::Result,
        chain_id::EthereumChainId,
    },
    interactive_cli_lib::{
        utils::get_user_input,
        state::InteractiveCliState,
    }
};

pub fn get_eth_network_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    let options = vec![ "Mainnet", "Kovan", "Goerli", "Ropsten", "Rinkeby" ];
    println!("❍ Please select which network to use:");
    options.iter().enumerate().map(|(i, option)| println!("{} - {}", i + 1, option)).for_each(drop);
    get_user_input()
        .and_then(|user_input| match user_input.parse::<usize>() {
            Err(_) => {
                println!("✘ Please select a number from the above options!");
                get_eth_network_from_user(state)
            }
            Ok(selection) => match selection > 0 && selection <= options.len() {
                false => {
                    println!("✘ Please select a number from 1 to {}!", options.len());
                    get_eth_network_from_user(state)
                }
                true => {
                    let option = options[selection - 1];
                    println!("✔ {}", option);
                    Ok(state.add_eth_network(EthereumChainId::from_str(option)?))
                }
            }
        })
}
