const advent = require('./advent.js');

const attempt1 = async () => {
  const data = await advent.getIntArray('day1.txt');

  const results = data.map((v, i) => {
    if (!data[i+2] || i === 0) return 0;
    const getSum = at => data[at] + data[at+1] + data[at+2];

    console.log([data[i], data[i+1], data[i+2]], [data[i-1], data[i], data[i+1]])
    console.log(getSum(i), getSum(i+1));
    return getSum(i-1) < getSum(i);
  }).reduce((acc, curr) => acc + curr, 0);
  console.log(results);
};

const attempt2 = async () => {
  const data = await advent.getIntArray('day1.txt');
  let acc = 0;
  for (let i = 0; i < data.length - 2; i++) {
    if (i === 0) continue;
    const thisSum = data[i] + data[i+1] + data[i+2];
    const thatSum = data[i-1] + data[i] + data[i+1];
    console.log(thisSum, thatSum);
    if (thisSum > thatSum) acc++;
  }
  console.log(acc);
};

const attempt3 = async () => {
  const data = await advent.getIntArray('day1.txt');

  const results = data.reduce((acc, curr, i) => {
      if (!data[i+2] || i === 0) return acc;
    const getSum = at => [0, 1, 2].reduce((acc2, curr2) => acc2 + data[at + curr2], 0);
    return acc + (getSum(i-1) < getSum(i));
  }, 0);
  console.log(results);
};


attempt3();