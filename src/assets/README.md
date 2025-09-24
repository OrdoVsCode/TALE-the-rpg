This folder contains simple SVG placeholder assets for the game.

Files:
- player.svg  — 32x32 green player placeholder
- enemy.svg   — 32x32 red enemy placeholder
- tile.svg    — 32x32 gray tile placeholder

Notes:
- ggez supports loading PNGs more directly; if you prefer PNGs, convert these with an image tool (ImageMagick or rsvg-convert).

Convert examples:

Using ImageMagick:

```bash
magick convert src/assets/player.svg src/assets/player.png
magick convert src/assets/enemy.svg src/assets/enemy.png
magick convert src/assets/tile.svg src/assets/tile.png
```

Using rsvg-convert:

```bash
rsvg-convert -w 32 -h 32 src/assets/player.svg -o src/assets/player.png
rsvg-convert -w 32 -h 32 src/assets/enemy.svg -o src/assets/enemy.png
rsvg-convert -w 32 -h 32 src/assets/tile.svg -o src/assets/tile.png
```

If you want, I can add code to load these images as textures and render them instead of colored rectangles — say the word and I'll wire them into `player.rs` / `enemy.rs` / `map.rs`.