const utils = require('./utils');

(async () => {
  console.log("Utility token examples");
  const web3 = utils.web3();
  let accounts = await web3.eth.getAccounts();
  console.log(`accounts: ${JSON.stringify(accounts)}`);
  console.log(`${accounts[0]} balance: ${await web3.eth.getBalance(accounts[0])}`);
  console.log(`${accounts[1]} balance: ${await web3.eth.getBalance(accounts[1])}`);

  // TODO modify contract address after deploy
  let contractAddress = "0x4866970be557faf3b2adc40d4dee60d0a6489e8a";

  let contract = new web3.eth.Contract(utils.abi, contractAddress);

  let result;

  result = await contract.methods.balanceOf(accounts[0]).call();
  console.log(`contract.balanceOf(${accounts[0]}) = ${result}`);

  await utils.provider.engine.stop();
})();
