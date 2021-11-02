const utils = require('./utils');

(async () => {
  console.log("Utility token examples");
  const web3 = utils.web3();
  let accounts = await web3.eth.getAccounts();
  console.log(`accounts: ${JSON.stringify(accounts)}`);
  console.log(`${accounts[0]} balance: ${await web3.eth.getBalance(accounts[0])}`);
  console.log(`${accounts[1]} balance: ${await web3.eth.getBalance(accounts[1])}`);

  // TODO modify contract address after deploy
  // let contractAddress = "0x4866970be557faf3b2adc40d4dee60d0a6489e8a";
  let contractAddress = "0xe903bc1ef72215a2e6a74b6f1693add99b3afa10";

  let result;
  // Normal balance of can not use anymore
  // let contract = new web3.eth.Contract(utils.abi, contractAddress);
  // result = await contract.methods.balanceOf(accounts[0]).call();
  // console.log(`contract.balanceOf(${accounts[0]}) = ${result}`);

  // We need to make a signed balanceOf, this is from admin, or from user

  // From user
  let contract = new web3.eth.Contract(utils.abi, contractAddress, { from: accounts[1] });
  result = await contract.methods.balanceOf(accounts[1]).call();
  console.log(`user check his self contract.balanceOf(${accounts[1]}) = ${result}`);

  // From admin
  contract = new web3.eth.Contract(utils.abi, contractAddress, { from: accounts[0] });
  result = await contract.methods.balanceOf(accounts[1]).call();
  console.log(`admin check contract.balanceOf(${accounts[1]}) = ${result}`);

  await utils.provider.engine.stop();
})();
