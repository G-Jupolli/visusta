# Sobel (Sobel-Feldman) Filter

The Sobel filter is an edge detection algorithm that uses the directional change \
in luminance value.

## Directional Filters

The Sobel operator uses two 3x3 kernels to calculate the gradient in the x and y directions.

**Kernel Coordinates**
```
The frame around center c using cardinal directions:

     ╭          ╮
     | nw  n ne |
A  = |  w  c  e |
     | sw  s se |
     ╰          ╯
```

**Sobel Kernels**
```
Where:
Gx = gradient along the x axis
Gy = gradient along the y axis

     ╭          ╮
     | -1  0  1 |
Gx = | -2  0  2 | * A
     | -1  0  1 |
     ╰          ╯

     ╭          ╮
     | -1 -2 -1 |
Gy = |  0  0  0 | * A
     |  1  2  1 |
     ╰          ╯
```

**Convolution Equations**
```
Gx = ne - nw + ( 2 * (e - w) ) + se - sw
Gy = sw + ( s * 2 ) + se - nw + ( n * 2 ) + ne
```

## Magnitude and Direction

From the gradient components, we can calculate the magnitude and direction of the edge.

**Formulas**
```
Where:
m = magnitude of the vector (Gx, Gy)
d = radial direction of (Gx, Gy), range: -pi <= d <= pi

m^2 = Gx^2 + Gy^2
d   = atan2(Gy, Gx)
```

## Parameters

| Parameter | Symbol | Description |
|-----------|--------|-------------|
| magnitude_min | t | Threshold on m^2, avoids expensive sqrt call |

## Practical Use

### Thresholding

Having a threshold on magnitude is used for de-noising. By comparing against m^2 \
instead of m, we avoid an expensive square root calculation.

### Colouring

We can assign colour to pixels based on any computed value.

Example: Setting the Red channel to `Gx * m` and the Blue channel to `Gy * m` \
results in an image where transitional pixels are coloured based on their edge direction.

### ASCII Rendering

By using the Sobel filter, we get the directional change in luminance of a pixel. \
We can use characters `|` `/` `-` `\` to represent directional edges.

**ASCII Parameters**
| Parameter | Symbol | Description |
|-----------|--------|-------------|
| font_size | f | Size of the sampling frame |
| ascii_max | tf | Minimum proportion of directional pixels in frame |
| chars | - | Character set for directions: `\|`, `/`, `-`, `\` |

**Process**
1. Define font size `f` to scan frames of size `f x f`
2. For each frame, count pixels by direction `d`
3. Assign the character corresponding to the most common direction
4. Apply threshold `tf` to filter out frames with too few directional pixels

## Limitations

The Sobel filter exposes edges on a per-pixel level. However, these include \
any pixels with transitional luminance. A face with a shadow will have a \
gradient across its surface, requiring a high threshold `t` to de-noise.

**Solution:** Use a Difference of Gaussians filter as an initial de-noising step. \
See [Gaussian.md](./Gaussian.md) for details.
