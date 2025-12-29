# Difference of Gaussians (DoG) Filter

The Difference of Gaussians filter is a band-pass filter that can be used for \
edge detection and noise reduction. It works by subtracting one blurred version \
of an image from another, less blurred version.

## Gaussian Blur

A Gaussian blur is calculated using a kernel derived from the Gaussian distribution.

**Gaussian Formula**
```
Where:
G  = Gaussian value at point (x, y)
pi = PI
s  = sigma (standard deviation)
x  = x coordinate relative to center
y  = y coordinate relative to center

        1         ╭  x^2 + y^2 ╮
G = ──────── exp  |- ───────── |
    2 pi s^2      ╰    2 s^2   ╯

Simplified:
Ga = 2 pi s^2
Gb = - ( x^2 + y^2 ) / 2 s^2

G = exp(Gb) / Ga
```

## 3x3 Kernel Calculation

Currently using a radius `r = 1` for a 3x3 kernel.

**Kernel Coordinates**
```
The kernel is calculated for coordinates around center c at (0, 0):

╭                   ╮
| -1,-1  0,-1  1,-1 |
| -1, 0  0, 0  1, 0 |
| -1, 1  0, 1  1, 1 |
╰                   ╯
```

**Kernel Symmetry**
```
Due to x and y being squared, the kernel has 1 + 2r unique values:

╭          ╮
| As Bs As |
| Bs Cs Bs |
| As Bs As |
╰          ╯

Where As, Bs, Cs are values dependent on sigma.
```

## Difference of Gaussians

The DoG is calculated by subtracting two normalised Gaussian kernels with \
different sigma values.

**Normalised Difference Formula**
```
Where:
Gaxy = point xy on the first Gaussian (smaller sigma)
Gbxy = point xy on the second Gaussian (larger sigma)
Sa   = sum of the first Gaussian
Sb   = sum of the second Gaussian
kxy  = resulting kernel value at xy

        Gaxy     Gbxy
kxy = ────── - ──────
        Sa       Sb
```

**With Scalar**
```
Where:
t = scalar (tau) for the second Gaussian

       Gaxy         Gbxy
kxy = ────── - t * ───────
        Sa           Sb
```

## Parameters

| Parameter | Symbol | Description |
|-----------|--------|-------------|
| sigma_a   | sa     | Standard deviation for the first (sharper) Gaussian |
| sigma_b   | sb     | Standard deviation for the second (blurrier) Gaussian |
| scalar    | t      | Optional scaling factor for the second Gaussian |
| cutoff    | e      | Optional threshold for the output values |

**Constraints**
- Both sigmas must be positive: `sa > 0` and `sb > 0`
- The second sigma must be larger: `sb > sa`

## Practical Use

### Edge Enhancement

The DoG filter enhances edges by subtracting the more blurred image from the \
less blurred one. This removes low-frequency information while preserving edges.

### Noise Reduction for Sobel

As mentioned in the Sobel documentation, the DoG filter can be used as a \
pre-processing step before applying the Sobel filter. This helps reduce noise \
and gradual luminance changes (like shadows on faces) that would otherwise \
create false edges.

### Binary Cutoff

The `cutoff` parameter can be used to create a binary output:
- Pixels with accumulated values above the cutoff are set to maximum
- This creates a clean edge map suitable for further processing
