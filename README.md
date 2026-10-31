# CHRtool

Command line utility for editing NES .chr files

Expects palette:

![alt text](image.png)

black/transparent
R: 0 G: 0 B: 0

dark grey
R: 102 G: 103 B: 101

light grey
R: 177 G: 177 B: 175

white
R: 255 G: 255 B: 255

# Use:

build chr:
```
chrtool -b image1.png image2.png
```
adds image1 to Pattern Table 0, image2 to Pattern Table 1
.png and .bmp accepted

open chr:
```
chrtool -o mychr.chr
```

# A note on AI contributions:

This project is intentionally written and maintained by humans only. 