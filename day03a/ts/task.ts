console.log(
  "Sum: ",
  (await Deno.readTextFile("input.txt"))
    .matchAll(/mul\((\d*),(\d*)\)/gm)
    .reduce((acc, capture) => {
      return (acc += +capture[1] * +capture[2]);
    }, 0),
);
