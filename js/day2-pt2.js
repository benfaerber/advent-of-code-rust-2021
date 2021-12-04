const advent = require('./advent.js');

const attempt1 = async () => {
  const data = await advent.getLines('day2.txt');

  // Horiziontal, depth, aim
  const result = data.reduce(([oldH, oldD, oldA], curr) => {
    const [cmd, rawValue] = curr.split(' ');
    const value = parseInt(rawValue);

    const hori = cmd === 'forward' ? value : 0;
    const depth = cmd === 'forward' ? (oldA * value) : 0;

    const scalar = cmd === 'down' ? 1 : -1;
    const aim = ['up', 'down'].includes(cmd) ? scalar * value : 0;

    return [
      oldH + hori,
      oldD + depth,
      oldA + aim
    ];
  }, [0, 0, 0]);

  const [h, d, a] = result;
  console.log(h * d);
};

const attempt2 = async () => {
  const data = await advent.getLines('day2.txt');

  // Horiziontal, depth, aim
  const result = data.reduce(([oldH, oldD, oldA], curr) => {
    const [cmd, rawValue] = curr.split(' ');
    const value = parseInt(rawValue);

    const hori = cmd === 'forward' ? value : 0;
    const depth = cmd === 'forward' ? (oldA * value) : 0;

    const scalar = cmd === 'down' ? 1 : -1;
    const aim = ['up', 'down'].includes(cmd) ? scalar * value : 0;

    return [
      oldH + hori,
      oldD + depth,
      oldA + aim
    ];
  }, [0, 0, 0]);

  const [h, d, a] = result;
  console.log(h * d);
};

attempt2();