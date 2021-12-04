const fs = require('fs').promises;

const firstAttempt = async () => {
  const fileHandler = await fs.readFile('./inputs/day1.txt');
  const rawData = fileHandler.toString();
  const data = rawData.split('\n').map(v => parseInt(v));

  const results = data
  .map((v, i) => i !== 0 ? data[i - 1] < v : 0)
  .reduce((acc, curr) => acc + curr);

  console.log(results);
};

const secondAttempt = async () => {
  const fileHandler = await fs.readFile('./inputs/day1.txt');
  const rawData = fileHandler.toString();
  const data = rawData.split('\n').map(v => parseInt(v));

  // uses the fact that true + true = 2
  const results = data.reduce((acc, curr, i) => (data[i - 1] < curr) + acc, 0);

  console.log(results);
};

secondAttempt();