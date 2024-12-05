const input: string[] = (await Deno.readTextFile("input.txt"))
  .trim()
  .split("\n");

const splitIndex = input.findIndex((item) => item === "");

const rules = input
  .slice(0, splitIndex)
  .map((item) => item.split("|").map(Number));

const sum = input
  .slice(splitIndex + 1, input.length)
  .map((item) => item.split(",").map(Number))
  .reduce((acc, numbers) => {
    const isCorrect = rules
      .filter((rule) => numbers.includes(rule[0]) && numbers.includes(rule[1]))
      .every((rule) => {
        const num1Index = numbers.findIndex((num) => num === rule[0]);
        const num2Index = numbers.findIndex((num) => num === rule[1]);

        return num1Index < num2Index;
      });

    if (!isCorrect) return acc;

    return acc + numbers[Math.floor(numbers.length / 2)];
  }, 0);

console.log(`Sum: ${sum}`);
