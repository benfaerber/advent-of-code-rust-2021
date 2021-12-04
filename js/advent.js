const fs = require('fs').promises;

const getIntArray = async filename => {
  const fileHandler = await fs.readFile(`./inputs/${filename}`);
  const rawData = fileHandler.toString();
  const data = rawData.split('\n').map(v => parseInt(v));
  return data;
};

const getLines = async filename => {
  const fileHandler = await fs.readFile(`./inputs/${filename}`);
  const rawData = fileHandler.toString();
  const data = rawData.split('\n');
  return data;
};

module.exports = {getIntArray, getLines};