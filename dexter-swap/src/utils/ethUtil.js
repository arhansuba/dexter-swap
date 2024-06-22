import { ethers } from 'ethers';

// Initialize Ethereum provider and signer
const provider = new ethers.providers.JsonRpcProvider(); // Replace with your provider URL if not using default
let signer;

// Function to initialize signer
export async function initializeSigner(privateKey) {
  signer = new ethers.Wallet(privateKey, provider);
  await signer.connect(provider);
}

// Function to get current Ethereum network
export async function getCurrentNetwork() {
  const network = await provider.getNetwork();
  return network;
}

// Function to get Ethereum balance of an address
export async function getBalance(address) {
  const balance = await provider.getBalance(address);
  return ethers.utils.formatEther(balance);
}

// Function to send Ether from signer's address to another address
export async function sendEther(toAddress, amount) {
  const tx = {
    to: toAddress,
    value: ethers.utils.parseEther(amount),
  };
  const txResponse = await signer.sendTransaction(tx);
  await txResponse.wait();
  return txResponse.hash;
}

// Function to call a read-only function on a contract
export async function callContractMethod(contract, methodName, params) {
  const method = contract.interface.functions[methodName];
  const result = await contract.callStatic[method.name](...params);
  return result;
}

// Function to send a transaction to a contract
export async function sendContractTransaction(contract, methodName, params) {
  const method = contract.interface.functions[methodName];
  const tx = {
    to: contract.address,
    data: contract.interface.encodeFunctionData(method, params),
  };
  const txResponse = await signer.sendTransaction(tx);
  await txResponse.wait();
  return txResponse.hash;
}

// Function to estimate gas cost of a transaction
export async function estimateGas(tx) {
  const gasEstimate = await provider.estimateGas(tx);
  return gasEstimate.toString();
}

// Function to format Ethereum address (e.g., add '0x' prefix)
export function formatAddress(address) {
  return ethers.utils.getAddress(address);
}

// Other utility functions can be added based on your application's requirements

export default {
  initializeSigner,
  getCurrentNetwork,
  getBalance,
  sendEther,
  callContractMethod,
  sendContractTransaction,
  estimateGas,
  formatAddress,
};
