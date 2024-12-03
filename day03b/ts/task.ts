let enabled = true;
console.log(
  "Sum: ",
  (await Deno.readTextFile("input.txt"))
    .matchAll(/mul\((\d*),(\d*)\)|do\(\)|don\'t\(\)/gm)
    .reduce((acc, capture) => {
      if (capture[0].startsWith("do()")) enabled = true;
      if (capture[0].startsWith("don't()")) enabled = false;
      if (capture[0].startsWith("mul(") && enabled)
        acc += +capture[1] * +capture[2];

      return acc;
    }, 0),
);
