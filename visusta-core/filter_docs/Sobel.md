# Sobel ( Sobel-Feldman ) Filter

The sobel filter is an edge detection algorithm that uses the directional change  \
in luminance value.

**The base Sobel Directional Filters**
```
Where:
Gx is the change along the x axis
Gy is the change along the y axis
A is the frame around center c as cardinal directions

     ╭          ╮
     | nw  n ne |
A  = |  w  c  e |
     | sw  s se |
     ╰          ╯

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

Expressing These convolutions as equations we get
```
Gx = ne - nw + ( 2 * (e - w) ) + se - sw
Gy = sw + ( s * 2 ) + se - nw + ( n * 2 ) + ne
```

Considering point ( Gx, Gy )
```
m is the magnitude of the vector (Gx, Gy)
d is the radial direction of (Gx, Gy) -pi <= d >= pi

m^2 = Gx^2 + Gy^2
d   = atan2(Gx, Gy)
```

## Practical Use

Having a threshold of magnitude is for de-noising. 
```
t is a threshold on m^2, avoids expensive sqrt call
```

### Colouring

We can assign colour to pixels based off any value.

e.g Setting the Red value of a pixel to Gx * m and the blue value to Gy * m  \
This would result in a picture where the transitional pixels are coloured.

### ASCII rendering

By using the sobel filter, we get the directional change in luminance of a pixel.  \
We can use characters `|` `/` `-` `\` to be our directional chars.  \
If we define font size `f` we can scan frames of size `f x f`.  \
Depending on the most common direction `d` of the pixels, we can assign them a char.

There will be a threshold of `tf` that dictates the minimum proportion of directional  \
pixels in an `f x f` frame.

### Limitations

The Sobel filter will expose edges on a per pixel level.  \
However, these will be any pixels of transitional luminance.  \
A face with a shadow will have a gradient across it's whole such that  \
a threshold `t` would have to be high enough to de-noise.

To solve this, we can use a Difference of Gaussians filter as an  \
initial de noising step.
