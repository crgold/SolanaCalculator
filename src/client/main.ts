/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  calculateInput,
  reportResult,
} from './calculator';

async function main() {
  console.log("Let's do some math...");

  let operation;
  let val1;
  let val2;

  if (process.argv.length !== 5) {
    throw new Error("You need to provide the operation and values you would like to use.");
  }

  // Get the arguments
  process.argv.forEach(function (val, index, array) {
    switch (index) {
      case 2: {
        operation = val; // operator
      }
      case 3: {
        val1 = parseInt(val); // first value
      }
      case 4: {
        val2 = parseInt(val); // second value
      }
    }
  });

  if (!operation || !val1 || !val2) {
    throw new Error("Invalid input.");
  }

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await calculateInput(operation, val1, val2);

  // Find out how many times that account has been greeted
  await reportResult(operation, val1, val2);

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
