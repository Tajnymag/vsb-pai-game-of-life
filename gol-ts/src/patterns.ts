import { RuleLengthEncoded } from './rel';

export const beehive = new RuleLengthEncoded(`#N Beehive
#O John Conway
#C An extremely common 6-cell still life.
#C www.conwaylife.com/wiki/index.php?title=Beehive
x = 4, y = 3, rule = B3/S23
b2ob$o2bo$b2o!`);

export const glider = new RuleLengthEncoded(`#N Glider
#O Richard K. Guy
#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.
#C www.conwaylife.com/wiki/index.php?title=Glider
x = 3, y = 3, rule = B3/S23
bob$2bo$3o!`);

export const block = new RuleLengthEncoded(`#N Block
#C An extremely common 4-cell still life.
#C www.conwaylife.com/wiki/index.php?title=Block
x = 2, y = 2, rule = B3/S23
2o$2o!`);

export const blinker = new RuleLengthEncoded(`#N Blinker
#O John Conway
#C A period 2 oscillator that is the smallest and most common oscillator.
#C www.conwaylife.com/wiki/index.php?title=Blinker
x = 3, y = 1, rule = B3/S23
3o!`);

export const rPentomino = new RuleLengthEncoded(`#N R-pentomino
#C A methuselah with lifespan 1103.
#C www.conwaylife.com/wiki/index.php?title=R-pentomino
x = 3, y = 3, rule = B3/S23
b2o$2ob$bo!`);

export const allPatterns = [beehive, glider, block, blinker, rPentomino];

export const patternColorsRGB = [
  [0, 0, 0],
  [255, 0, 0],
  [0, 255, 0],
  [0, 0, 255],
  [255, 255, 0],
  [255, 0, 255],
  [0, 255, 255],
  [255, 255, 255],
];
