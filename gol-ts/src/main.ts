import 'bootstrap/dist/css/bootstrap.min.css';
import './style.css';

import * as Comlink from 'comlink';

import GoLWorkerNative from './gol-worker?worker';
import { GameBoard, GoLWorker } from './gol-worker';
import { RuleLengthEncoded } from './rel';
import { clearArray } from "./utils";
import { patternColorsRGB } from "./patterns";

async function iterate(workers: Comlink.Remote<GoLWorker>[], board: Omit<GameBoard, 'data'>) {
	let jobs: Promise<unknown>[] = [];

	const workersCount = workers.length;
	const cellsCount = board.width * board.height;

	const chunkSize = Math.floor(cellsCount / workersCount);
	const oddChunks = cellsCount % workersCount;

	workers.forEach((worker, workerIndex) => {
		const from = workerIndex * chunkSize;
		const to = from + chunkSize + (workerIndex < oddChunks ? 1 : 0);
		jobs.push(worker.processCells(from, to));
	});

	await Promise.all(jobs);
}

async function detectPatterns(workers: Comlink.Remote<GoLWorker>[], board: Omit<GameBoard, 'data'>, patternBuffer: SharedArrayBuffer) {
  let jobs: Promise<unknown>[] = [];

  const workersCount = workers.length;
  const cellsCount = board.width * board.height;

  const chunkSize = Math.floor(cellsCount / workersCount);
  const oddChunks = cellsCount % workersCount;

  workers.forEach((worker, workerIndex) => {
    const from = workerIndex * chunkSize;
    const to = from + chunkSize + (workerIndex < oddChunks ? 1 : 0);
    jobs.push(worker.detectPatterns(from, to, patternBuffer));
  });

  await Promise.all(jobs);
}

async function swapBuffers(workers: Comlink.Remote<GoLWorker>[]) {
	const jobs: Promise<unknown>[] = [];

	workers.forEach((worker) => {
		jobs.push(worker.swapBuffers());
	});

	await Promise.all(jobs);
}

async function renderBoard(board: GameBoard, patternBoard: GameBoard, ctx: CanvasRenderingContext2D) {
	const imageData = new ImageData(board.width, board.height);

	for (let i = 0; i + 4 < imageData.data.length; i += 4) {
		const cellPosition = Math.floor(i / 4);
		const cellValue = Atomics.load(board.data, cellPosition);
    const patternValue = Atomics.load(patternBoard.data, cellPosition);

		let color: [number, number, number, number];

		if (cellValue === 1) {
      const r = patternColorsRGB[patternValue][0];
      const g = patternColorsRGB[patternValue][1];
      const b = patternColorsRGB[patternValue][2];
			color = [r, g, b, 255];
		} else {
			color = [255, 255, 255, 255];
		}

		imageData.data[i] = color[0];
		imageData.data[i + 1] = color[1];
		imageData.data[i + 2] = color[2];
		imageData.data[i + 3] = color[3];
	}

	ctx.putImageData(imageData, 0, 0);
}

async function play({
	signal,
	framerate,
	width,
	height,
	canvas,
	seed,
}: {
	signal: AbortSignal;
	framerate: number;
	width: number;
	height: number;
	canvas: HTMLCanvasElement;
	seed: RuleLengthEncoded;
}) {
	const workersNative: Worker[] = [];
	const workers: Comlink.Remote<GoLWorker>[] = [];
	const workersCount = navigator.hardwareConcurrency;

	const ctx = canvas.getContext('2d')!;

	canvas.width = width;
	canvas.height = height;

	const sharedBufferA = new SharedArrayBuffer(width * height);
	const sharedBufferB = new SharedArrayBuffer(width * height);
  const patternBuffer = new SharedArrayBuffer(width * height);

	const gameBoard: GameBoard = {
		data: new Uint8Array(sharedBufferA),
		width: width,
		height: height,
	};

  const patternBoard: GameBoard = {
    data: new Uint8Array(patternBuffer),
    width: width,
    height: height,
  };

	const startX = Math.floor((gameBoard.width - seed.width) / 2);
	const startY = Math.floor((gameBoard.height - seed.height) / 2);
	for (let y = 0; y < seed.height; ++y) {
		for (let x = 0; x < seed.width; ++x) {
			const boardIndex = gameBoard.width * startY + startX + gameBoard.width * y + x;

			if (boardIndex >= gameBoard.data.length) continue;

			gameBoard.data[boardIndex] = seed.data[y][x];
		}
	}

	for (let workerIndex = 0; workerIndex < workersCount; ++workerIndex) {
		if (signal.aborted) return;

		const workerNative = new GoLWorkerNative();
		const WrappedGoLWorker = Comlink.wrap<typeof GoLWorker>(workerNative);
		const worker = await new WrappedGoLWorker(sharedBufferA, sharedBufferB, {
			width: gameBoard.width,
			height: gameBoard.height,
		});

		workersNative.push(workerNative);
		workers.push(worker);
	}

	let iteration = 0;
	let lastTimestamp = 0;
	const renderFrame = async (timestamp: number) => {
		if (signal.aborted) return;
		const delta = timestamp - lastTimestamp;

		if (delta >= 1000 / framerate) {
      // cell life iteration
			await iterate(workers, {
				width: gameBoard.width,
				height: gameBoard.height,
			});

      // pattern detection
      clearArray(patternBoard.data);
      await detectPatterns(workers, {
        width: gameBoard.width,
        height: gameBoard.height,
      }, patternBuffer);

      // canvas rendering
			await renderBoard(gameBoard, patternBoard, ctx);

      // game buffer swap
			gameBoard.data = new Uint8Array(++iteration % 2 === 0 ? sharedBufferA : sharedBufferB);
			await swapBuffers(workers);

			lastTimestamp = timestamp;
		}

		requestAnimationFrame(renderFrame);
	};

	signal.onabort = () => {
		workersNative.forEach((worker) => worker.terminate());
	};

	requestAnimationFrame(renderFrame);
}

async function main() {
	const mainCanvasEl = document.getElementById('main-canvas') as HTMLCanvasElement;
	const patternTextAreaEl = document.getElementById('pattern-textarea') as HTMLTextAreaElement;
	const boardWidthInputEl = document.getElementById('board-width-input') as HTMLInputElement;
	const boardHeightInputEl = document.getElementById('board-height-input') as HTMLInputElement;
	const targetFramerateInputEl = document.getElementById('target-framerate-input') as HTMLInputElement;
	const targetFramerateLabel = document.querySelector('label[for=target-framerate-input]') as HTMLLabelElement;
	const playBtnEl = document.getElementById('play-btn') as HTMLButtonElement;

	let controller = new AbortController();
	let isRunning = false;

	targetFramerateInputEl.oninput = (ev) => {
		targetFramerateLabel.innerText = `Target framerate: ${(ev.target as HTMLInputElement).value}`;
	};

	playBtnEl.onclick = (ev) => {
		ev.preventDefault();

		if (isRunning) {
			controller.abort();
			controller = new AbortController();
		}

		isRunning = true;
		play({
			signal: controller.signal,
			width: parseInt(boardWidthInputEl.value),
			height: parseInt(boardHeightInputEl.value),
			framerate: parseInt(targetFramerateInputEl.value),
			canvas: mainCanvasEl,
			seed: new RuleLengthEncoded(patternTextAreaEl.value),
		});
	};
}
main().catch((err) => console.error(err));
