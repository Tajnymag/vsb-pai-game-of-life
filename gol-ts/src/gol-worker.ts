import * as Comlink from 'comlink';
import { allPatterns } from "./patterns";

export type Coordinate = number;

export interface GameBoard {
	data: Uint8Array;
	width: number;
	height: number;
}

export class GoLWorker {
	private sourceBoard: GameBoard;
	private targetBoard: GameBoard;

	constructor(
		sharedSourceBuffer: SharedArrayBuffer,
		sharedTargetBuffer: SharedArrayBuffer,
		board: Omit<GameBoard, 'data'>
	) {
		this.sourceBoard = {
			...board,
			data: new Uint8Array(sharedSourceBuffer),
		};
		this.targetBoard = {
			...board,
			data: new Uint8Array(sharedTargetBuffer),
		};
	}

	async reset(
		sharedSourceBuffer: SharedArrayBuffer,
		sharedTargetBuffer: SharedArrayBuffer,
		board: Omit<GameBoard, 'data'>
	) {
		this.sourceBoard = {
			...board,
			data: new Uint8Array(sharedSourceBuffer),
		};
		this.targetBoard = {
			...board,
			data: new Uint8Array(sharedTargetBuffer),
		};
	}

	async swapBuffers() {
		[this.sourceBoard, this.targetBoard] = [this.targetBoard, this.sourceBoard];
	}

	async processCells(from: Coordinate, to: Coordinate) {
		for (let cellIndex = from; cellIndex < to; ++cellIndex) {
			const x = cellIndex % this.sourceBoard.width;
			const y = Math.trunc((cellIndex - x) / this.sourceBoard.width);

			let cellAlive = Atomics.load(this.sourceBoard.data, cellIndex);

			let neighborsCount = 0;

			for (let neighborOffsetX = -1; neighborOffsetX <= 1; ++neighborOffsetX) {
				for (let neighborOffsetY = -1; neighborOffsetY <= 1; ++neighborOffsetY) {
					if (neighborOffsetX === 0 && neighborOffsetY === 0) continue;
					const neighborX = x + neighborOffsetX;
					const neighborY = y + neighborOffsetY;

					if (neighborX < 0 || neighborX >= this.sourceBoard.width) continue;
					if (neighborY < 0 || neighborY >= this.sourceBoard.height) continue;

					const neighborIndex = this.sourceBoard.width * neighborY + neighborX;
					const neighborAlive = Atomics.load(this.sourceBoard.data, neighborIndex);

					if (neighborAlive) {
						neighborsCount += 1;
					}
				}
			}

			if (cellAlive && neighborsCount < 2) {
				cellAlive = 0;
			} else if (cellAlive && neighborsCount <= 3) {
				cellAlive = 1;
			} else if (cellAlive && neighborsCount > 3) {
				cellAlive = 0;
			} else if (!cellAlive && neighborsCount === 3) {
				cellAlive = 1;
			}

			Atomics.store(this.targetBoard.data, cellIndex, cellAlive);
		}
	}

  detectPatterns(from: Coordinate, to: Coordinate, outputBuffer: SharedArrayBuffer) {
    const patternBoard: GameBoard = {
      data: new Uint8Array(outputBuffer),
      width: this.targetBoard.width,
      height: this.targetBoard.height
    };

    for (let i = from; i < to; ++i) {
      patternLoop:
      for (const [patternID, pattern] of allPatterns.entries()) {
        let patternStart = i;
        let patternEnd = patternStart + (pattern.height - 1) * this.targetBoard.width + pattern.width - 1;
        for (let patternY = 0; patternY < pattern.height; ++patternY) {
          for (let patternX = 0; patternX < pattern.width; ++patternX) {
            const cellIndex = patternStart + patternY * this.targetBoard.width + patternX;

            if (cellIndex >= this.targetBoard.data.length) {
              continue patternLoop;
            }

            const patternValue = pattern.data[patternY][patternX];
            const cellValue = Atomics.load(this.targetBoard.data, cellIndex);

            if (cellValue !== patternValue) {
              continue patternLoop;
            }
          }
        }

        for (let patternY = 0; patternY < pattern.height; ++patternY) {
          for (let patternX = 0; patternX < pattern.width; ++patternX) {
            const cellIndex = patternStart + patternY * this.targetBoard.width + patternX;
            Atomics.store(patternBoard.data, cellIndex, patternID);
          }
        }
      }
    }
  }
}

Comlink.expose(GoLWorker);
