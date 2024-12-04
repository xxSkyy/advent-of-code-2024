const findXmassToDir = (
  input: string[][],
  x: number,
  y: number,
  dirX: number,
  dirY: number,
) => {
  return (
    input[y][x] === "X" &&
    input?.[y + dirY]?.[x + dirX] === "M" &&
    input?.[y + dirY * 2]?.[x + dirX * 2] === "A" &&
    input?.[y + dirY * 3]?.[x + dirX * 3] === "S"
  );
};

const input = (await Deno.readTextFile("input.txt"))
  .split("\n")
  .map((item) => item.split(""));

console.log(
  "Xmases: ",
  input.reduce((sum, xLine, y) => {
    xLine.forEach((_, x) => {
      if (findXmassToDir(input, x, y, 1, 1)) sum++;
      if (findXmassToDir(input, x, y, 1, 0)) sum++;
      if (findXmassToDir(input, x, y, 1, -1)) sum++;
      if (findXmassToDir(input, x, y, 0, -1)) sum++;
      if (findXmassToDir(input, x, y, -1, -1)) sum++;
      if (findXmassToDir(input, x, y, -1, 0)) sum++;
      if (findXmassToDir(input, x, y, -1, 1)) sum++;
      if (findXmassToDir(input, x, y, 0, 1)) sum++;
    });
    return sum;
  }, 0),
);
