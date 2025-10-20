## What I'm trying to do:

There are 2 purposes I want to accomplish with this project:
 1. Cli that has some image filters that can process
 2. Process Image to ascii for site

## Filters:

For the filters, I want to have a compose-able system.  \
Meaning that the user can chose to do filter x -> z -> y.  \
I'm not too sure if I care about disallowing invalid states.

The filters I am looking at implementing right now:
 - Dithering
 - To ASCII ( Image reference, this is only or not at all )
 - Edge Detection ( Sobel )
 - Desaturation
 - Downscale

There is a 2 level process to the idea:
 1. Filters can be configured manually & Saved ( saving can be later )
 2. Some presets to quick process an image e.g.
    - Pencil Sketch
    - Pixelation
    - To ASCII

Others I'm thinking about:
 - Film Grain
 - Vignette
 - Bloom
 - Mosaic ( Could try to do a to Minecraft Filer )
 - Low poly ( Voronoi / Delaunay )

## CLI

The ain point of having this available as a cli is that I want it to be  \
 available for Astrid to use.  \
The ideal filter composition would be:  \
`visusta path/to/file -f downscale -f dither -f desaturate -o path/to/output`

Ideally the filters would maintain order, maybe easier to do comma separated list.  \
`-o` is just where to place the file, will default to current dir.

## 'ASCII for Site'

I watched this video a while ago:  \
https://www.youtube.com/watch?v=gg40RWiaHRY

In the video, they create a shader that can render games in ASCII art.  \
Instead of making a shader, I want to be able to process an image to ASCII  \
and then display the raw text "image" on a site.

In stripping it down like this, I don't need to care about color / bloom.  \
I just need to be able to get the text and present it in a mono font.
