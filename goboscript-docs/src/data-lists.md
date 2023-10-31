# Data Lists

Include the contents of a text file as a list, each line being an item in the list.

```goboscript
datalist list_name "path/to/text-file.txt";
```

# Image Lists

Include the pixel data of an image file as a list, each pixel being an item in the
list.

```goboscript
imagelist list_name "image-file.png";
```

The default format is to generate one item for each RGBA channel in the image.
