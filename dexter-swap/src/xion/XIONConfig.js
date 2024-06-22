// Example usage in xion.js or any other file

// Import configuration constants
const {
    XION_NETWORK,
    XION_RPC_URL,
    XION_CONTRACT_ADDRESS,
    XION_TOKEN_SYMBOL,
    XION_DECIMALS,
} = require('./xion-config');

// Use the constants as needed
console.log(`Connecting to ${XION_NETWORK} network with RPC URL: ${XION_RPC_URL}`);
console.log(`Xion token symbol: ${XION_TOKEN_SYMBOL}, Decimals: ${XION_DECIMALS}`);
console.log(`Using contract address: ${XION_CONTRACT_ADDRESS}`);

// You can use these constants in initializing connections, setting up contracts, or any other configuration-based logic.
