const rotateInput = (input: string[][]): string[][] => {
  const size = input.length;
  const rotatedInput: string[][] = Array.from({ length: size }, () =>
    Array(size),
  );

  for (let row = 0; row < size; row++) {
    for (let col = 0; col < size; col++) {
      rotatedInput[col][size - row - 1] = input[row][col];
    }
  }

  return rotatedInput;
};

const findXmassToDir = (input: string[][], x: number, y: number) => {
  return (
    input[y][x] === "M" &&
    input?.[y + 2]?.[x] === "M" &&
    input?.[y + 1]?.[x + 1] === "A" &&
    input?.[y]?.[x + 2] === "S" &&
    input?.[y + 2]?.[x + 2] === "S"
  );
};

let input = (await Deno.readTextFile("input.txt"))
  .split("\n")
  .map((item) => item.split(""));

let sum = 0;

for (let i = 0; i < 4; i++) {
  input = rotateInput(input);

  sum += input.reduce((sum, xLine, y) => {
    xLine.forEach((_, x) => {
      if (findXmassToDir(input, x, y)) sum++;
    });
    return sum;
  }, 0);
}

console.log("Mases: ", sum);
