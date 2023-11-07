use crate::errors::Result;
use crate::blockchain::Blockchain;
use std::process::exit;
use clap::{arg, Command};

pub struct Cli  {

}

impl Cli {
    pub fn new()-> Result<Cli>{
        Ok(Cli {

        })
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("acyrntoine@gmail.com")
            .about("simple blockchain state")
            .subcommand(Command::new("balance")
                .about("get balance in the blochain")
                .arg(arg!(<ADDRESS>"'The Address it get balance for'"))
        )
            .subcommand(Command::new("start-node").about("Create new blokchain")
            )
            .subcommand(
                Command::new("transfer")
                    .about("trasnfer in the blockchain")
                    .arg(arg!(<FROM>" 'Source address'"))
                    .arg(arg!(<TO>" 'Destination address'"))
                    .arg(arg!(<AMOUNT>" 'Amount'"))
            )
            .subcommand(
                Command::new("create-account")
                    .about("create a new account")
                    .arg(arg!(<ID>" 'address'"))
                    .arg(arg!(<AMOUNT>" 'Amount'"))
            )
            .get_matches();

        if let Some(ref _matches) = matches.subcommand_matches("start-node") {
            let bc= Blockchain::create_blockchain()?;
            bc.run_blockchain()?;
            println!("create blockchain");
        }

        if let Some(ref matches) = matches.subcommand_matches("balance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let balance = bc.get_balance(address.clone());
                println!("Balance of '{}'; {} ", address,balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("transfer") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            bc.add_transaction(from.clone(),to.clone(),amount)?;
            println!("transaction added to mempool!");
        }


        if let Some(ref matches) = matches.subcommand_matches("create-account") {

            let id = if let Some(address) = matches.get_one::<String>("ID") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            bc.add_transaction(String::from(""),id.clone(),amount)?;
            println!("user added")
        }

        Ok(())
        //keep in memory the blockchain
    }


}