const fs = require("fs");

function readCanto(lang = "en", num = 1) {
  const canto = fs
    .readFileSync(`txt/chapters/${lang}/${num}.txt`)
    .toString("utf-8")
    .split("\n");
  const cantoArr = [[]];
  for (const line of canto) {
    if (line === "") {
      cantoArr.push([]);
    } else {
      const index = cantoArr.length - 1;
      cantoArr[index].push(line.trim());
    }
  }
  return cantoArr;
}

for (let i = 1; i < 35; i++) {
  const it = readCanto("it", i);
  const en = readCanto("en", i);
  fs.writeFileSync(
    `json/chapters/${i}.json`,
    JSON.stringify({ it, en }, undefined, 0)
  );
}
