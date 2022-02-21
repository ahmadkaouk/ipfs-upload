# IPFS Upload & Contract Registry
A command line utility to upload a file to IPFS and store the CID in a smart contract written in Solidity

## Requirements
- A running IPFS Node
- A  developpment environment to deploy the smart contract (Hardhat, truffle ...)

## Example
To test the program locally, start by running an IPFS node:
```bash
ipfs daemon
```
In another terminal, start a JSON-RPC node (ex: Hardhat):
```bash
npx hardhat node
```
finally, launch the ipfs-upload with the path to file to upload. You will be asked to enter the
account address to use to deploy the smart contract and the URL to the JSON-RPC node.
```bash
> ipfs-upload --file tests/test.txt
Uploading tests/test.txt to IPFS...
File uploaded CID = QmQ6zADQZGASCnYxXfFkGHUF5Fc7ibrtjGWaxjY8w9nANp
Enter your account address(prefix with 0x:
0xbda5747bfd65f08deb54cb465eb87d40e51b197e
Enter the URL of RPC-JSON Server:
http://127.0.0.1:8545/
Contract deployed: address 0x538d…e241
```
