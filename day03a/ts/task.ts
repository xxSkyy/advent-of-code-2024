console.log(
  "Sum: ",
  (await Deno.readTextFile("input.txt"))
    .matchAll(/mul\((\d*),(\d*)\)/gm)
    .reduce((acc, capture) => {
      acc += +capture[1] * +capture[2];
      return acc;
    }, 0),
);
