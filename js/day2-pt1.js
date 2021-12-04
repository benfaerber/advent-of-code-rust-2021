const advent = require('./advent.js');

const attempt1 = async () => {
  const data = await advent.getLines('day2.txt');

  // Horizontal, depth
  const [horizontal, depth] = data.reduce((acc, curr, i) => {
    const [oldH, oldD] = acc;
    const [cmd, rawValue] = curr.split(' ');
    const value = parseInt(rawValue);

    const hori = cmd === 'forward' ? value : 0;

    const scalar = cmd === 'down' ? 1 : -1;
    const isDepth = cmd === 'down' || cmd === 'up';
    const veri = isDepth ? scalar * value : 0;

    return [
      oldH + hori,
      oldD + veri
    ];
  }, [0, 0]);

  console.log(horizontal, depth);
  console.log(horizontal * depth);
};

attempt1();