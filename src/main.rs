use clap::Parser;
use std::str::FromStr;

use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::fs::File;
use web3::{contract::Contract, types::Address};

mod error;
use error::ErrorKind;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to upload
    #[clap(short, long)]
    file: String,
}

#[tokio::main]
async fn main() {
    // Get the path of the file to upload from the arguments
    let args = Args::parse();
    // upload the file to ipfs
    match upload(&args.file).await {
        Ok(cid) => store_and_deploy(&cid).await.unwrap(),
        Err(_) => println!("File could not be uploaded"),
    };
}

/// Upload a file to ipfs and returns the genretade CID if success
async fn upload(file_path: &str) -> Result<String, ErrorKind> {
    match File::open(file_path) {
        Ok(file) => {
            let client = IpfsClient::default();
            println!("Uploading {} to IPFS...", file_path);

            match client.add(file).await {
                Ok(file) => {
                    println!("File uploaded CID = {}", file.hash);
                    Ok(file.hash)
                }
                Err(_) => Err(ErrorKind::FileNotUploaded),
            }
        }
        Err(_) => {
            println!("Could not found {}", file_path);
            Err(ErrorKind::FileNotFound)
        }
    }
}

/// store the cid in a smart contract and deploy it
async fn store_and_deploy(cid: &str) -> Result<(), ErrorKind> {
    // Read Account Address
    let mut address = String::new();
    println!("Enter your account address(prefix with 0x:");
    std::io::stdin().read_line(&mut address).unwrap();
    let address = Address::from_str(&address[..]).expect("Invalid Address");

    // Read JSON-RPC server url
    let mut url = String::new();
    println!("Enter the URL of RPC-JSON Server:");
    std::io::stdin().read_line(&mut url).unwrap();

    // Store the CID in a smart contract
    let http =
        web3::transports::Http::new(&url).expect("error connecting to url");
    let web3 = web3::Web3::new(http);

    // Deploy the smart contract
    let bytecode = include_str!("../contracts/CIDStorage.bin");
    if let Ok(contract) =
        Contract::deploy(web3.eth(), include_bytes!("../contracts/CIDStorage.abi"))
            .unwrap()
            .confirmations(0)
            .execute(bytecode, cid.to_owned(), address)
            .await
    {
        println!("Contract deployed: address {}", contract.address());
        Ok(())
    } else {
        Err(ErrorKind::ContractNotDeployed)
    }
}
