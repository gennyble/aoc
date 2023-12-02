import { open } from 'node:fs/promises';

let ln = 0;
try {
	let file = await open("input/day1");
	let content = await file.readFile({ "encoding": "utf-8" });
	file.close();

	let lines = content.split('\n');

	let sum = 0;
	for (const line of lines) {

		if (line.length == 0) {
			continue;
		}

		let tens = undefined;
		let ones = undefined;

		for (let i = 0; i < line.length; ++i) {
			let charCode = line.charCodeAt(i);
			let digited = charCode - 0x30;

			if (digited >= 0 && digited < 10) {
				if (!tens) {
					tens = digited;
				} else {
					ones = digited;
				}
			}
		}

		if (!tens) {
			throw new Error("tens not set!")
		} else {
			if (!ones) {
				ones = tens;
			}

			let num = tens * 10 + ones;
			sum += num;
		}

		ln++;
	}

	console.log(sum);
} catch (err) {
	console.log(`[ln:${ln}]: ${err}`);
}