/* Goboscript Standard Library 

The code in this file is added into every sprite. */

macro stdDistance x1, y1, x2, y2
  -> sqrt((!x2 - !x1) * (!x2 - !x1) + (!y2 - !y1) * (!y2 - !y1));

macro stdMagnitude x, y
  -> sqrt(!x * !x + !y * !y);

macro stdRGB R, G, B
  -> 65536 * !R + 256 * !G + !B;

macro stdRGBA R, G, B
  -> 16777216 * !A + !stdRGB(!R, !G, !B);

macro stdMin a, b
  -> !a - (!a - !b) * (!a > !b);

macro stdMax a, b
  -> !a + (!b - !a) * (!b > !a);

macro stdDot x1, y1, x2, y2
  -> !x1 * !x2 + !y1 * !y2;
