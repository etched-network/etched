import { Hex, http } from 'viem'
import { createWalletClient } from 'viem'
import { mainnet } from 'viem/chains'
import { privateKeyToAccount } from 'viem/accounts'
import { Settings } from './settings'
import { EtchedFn, EtchedPayload } from './types'
import { TransactionRequest, ethers } from 'ethers';
 
const client = createWalletClient({
  chain: mainnet,
  transport: http('http://127.0.0.1:8546')
})
const provider = new ethers.JsonRpcProvider('http://127.0.0.1:8546');
const ethersWallet = new ethers.Wallet(Settings.get('PRIVATE_KEY'), provider);

const account =  privateKeyToAccount(Settings.get('PRIVATE_KEY') as Hex);
let ethedNonce = 12
const main = async () => {
  const etched: TransactionRequest = {
    chainId: 2727,
    to: "0x25637F50C42318A182dD77bE25dE444422811552",
    value: ethers.parseEther("4.2069"),
    nonce: ethedNonce++,
    gasLimit: 100000n,
    gasPrice: ethers.parseUnits('1', 'gwei'),
  };
  const signedTx = await ethersWallet.signTransaction(etched);

  console.log(signedTx)
  const etchedTx: EtchedPayload = {
    fn: EtchedFn.EVM_CALL,
    input: signedTx,
  }

  // submit the etched transactions onchain as calldata
  const data = Buffer.from(JSON.stringify(etchedTx), 'utf8').toString('hex')
  const hash = await client.sendTransaction({ 
    account: account,
    to: '0xfF00000000000000000000000000000000000020',
    data: `0x${data}`,
    value: 0n,
  })
  console.log(`TX hash | ${ethedNonce}:${hash}`)
}


main()