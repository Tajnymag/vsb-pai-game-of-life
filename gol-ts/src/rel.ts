function isNumeric(token: string) {
	return '1234567890'.split('').includes(token);
}

export class RuleLengthEncoded {
	name = 'unknown';
	comment = '';
	createdBy = 'unknown';
	rule = '23/3';

	width = 0;
	height = 0;
	data: number[][] = [];

	constructor(stringFormat: string) {
		const lines = stringFormat
			.split('\n')
			.filter((l) => !!l)
			.map((l) => l.trim());
		let encodedPattern = '';

		let headerSet = false;
		const headerRegex = /x\s*=\s*(?<width>\d+)\s*,\s*y\s*=\s*(?<height>\d+)\s*(?:,\s*rule\s*=\s*(?<rule>[\w\/]+))?/;

		for (const line of lines) {
			if (line.startsWith('#C') || line.startsWith('#c')) {
				this.comment += line.replace(/^#C/, '').trim();
			} else if (line.startsWith('#N')) {
				this.name = line.replace(/^#N/, '').trim();
			} else if (line.startsWith('#O')) {
				this.createdBy = line.replace(/^#O/, '').trim();
			} else if (line.startsWith('#r')) {
				this.rule = line.replace(/^#r/, '');
			} else if (line.startsWith('#')) {
				console.warn(`Ignoring comment line: ${line}`);
			} else if (headerRegex.test(line)) {
				const match = line.match(headerRegex)!;

				const width = parseInt(match.groups?.width ?? '0');
				const height = parseInt(match.groups?.height ?? '0');
				const rule = match.groups?.rule ?? '23/3';

				this.width = width;
				this.height = height;
				this.rule = rule;
				headerSet = true;
			} else if (headerSet) {
				encodedPattern += line.trim();
			} else {
				throw new Error(`Encountered an unexpectedly formatted line!\n${line}`);
			}
		}

		let lastToken = '';
		let tagCount = 1;
		let decodedPatternLine: number[] = [];

		for (let i = 0; i < encodedPattern.length; ++i) {
			const token = encodedPattern[i];

			if (isNumeric(token)) {
				if (isNumeric(lastToken)) {
					tagCount = tagCount * 10 + parseInt(token);
				} else {
					tagCount = parseInt(token);
				}
			} else if (token === 'o' || token === 'b') {
				for (let c = 0; c < tagCount; ++c) {
					decodedPatternLine.push(token === 'o' ? 1 : 0);
				}
				tagCount = 1;
			} else if (token === '$') {
				for (let c = 0; c < tagCount; ++c) {
					this.data.push(decodedPatternLine);
					decodedPatternLine = [];
				}
				tagCount = 1;
			} else if (token === '!') {
				if (lastToken !== '$') this.data.push(decodedPatternLine);
				break;
			} else {
				throw new Error(`Unexpected token ${token} encountered on line ${i} of pattern`);
			}

			lastToken = token;
		}

		// make sure the line is exactly the specified width
		for (const patternLine of this.data) {
			while (patternLine.length > this.width) {
				patternLine.pop();
			}
			while (patternLine.length < this.width) {
				patternLine.push(0);
			}
		}

		if (this.data.length !== this.height) {
			throw new Error('Encoded pattern does not match specified dimensions!');
		}
	}
}
