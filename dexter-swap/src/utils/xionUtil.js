// xionutil.js

const Xion = require('xion'); // Xion paketini içeri aktaralım

// Xion nesnesini oluşturalım
const xion = new Xion();

/**
 * Akıllı sözleşme yükleme işlevi.
 * @param {object} abi - Sözleşme ABI'si.
 * @param {string} address - Sözleşme adresi.
 * @returns {Promise<object>} - Yüklü sözleşme nesnesi.
 */
async function loadContract(abi, address) {
    try {
        const contract = new xion.Contract(abi, address);
        return contract;
    } catch (error) {
        console.error('Contract loading error:', error);
        throw error;
    }
}

/**
 * Akıllı sözleşme yöntemini çağırma işlevi.
 * @param {object} contract - Sözleşme nesnesi.
 * @param {string} methodName - Çağrılacak yöntem adı.
 * @param {...any} params - Yönteme geçirilecek parametreler.
 * @returns {Promise<any>} - Yöntem çağrısının sonucu.
 */
async function callContractMethod(contract, methodName, ...params) {
    try {
        const result = await contract.methods[methodName](...params).call();
        return result;
    } catch (error) {
        console.error('Contract method call error:', error);
        throw error;
    }
}

/**
 * Xion ağı üzerinde işlem gönderme işlevi.
 * @param {string} from - Gönderen hesap adresi.
 * @param {string} to - Alıcı hesap adresi.
 * @param {string} value - Gönderilecek miktar.
 * @returns {Promise<object>} - İşlem nesnesi.
 */
async function sendTransaction(from, to, value) {
    try {
        const tx = await xion.sendTransaction({
            from: from,
            to: to,
            value: value,
        });
        return tx;
    } catch (error) {
        console.error('Transaction sending error:', error);
        throw error;
    }
}

/**
 * Belirli bir kullanıcının bakiyesini almak için işlev.
 * @param {string} address - Kullanıcı adresi.
 * @returns {Promise<string>} - Kullanıcının bakiyesi.
 */
async function getUserBalance(address) {
    try {
        const balance = await xion.getBalance(address);
        return balance.toString();
    } catch (error) {
        console.error('Error fetching user balance:', error);
        throw error;
    }
}

/**
 * Yeni bir hesap oluşturmak için işlev.
 * @param {string} passphrase - Yeni hesap için şifre.
 * @returns {Promise<object>} - Oluşturulan hesap nesnesi.
 */
async function createAccount(passphrase) {
    try {
        const account = await xion.createAccount(passphrase);
        return account;
    } catch (error) {
        console.error('Error creating account:', error);
        throw error;
    }
}

// Diğer özelleştirilmiş işlevleri ve Xion ağına özgü ayarlamaları burada ekleyebilirsiniz.

// Modüldeki tüm işlevleri dışa aktaralım
module.exports = {
    loadContract,
    callContractMethod,
    sendTransaction,
    getUserBalance,
    createAccount,
    // Diğer işlevler buraya eklenebilir
};

