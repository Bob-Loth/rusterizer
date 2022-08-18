# rusterizer

A simple, CLI-based rasterizer, adapted from a C++ project as a learning exercise for the Rust programming language.

Takes in a mesh file, describing vertex and triangle data, and outputs a 2D image of that data by:
1. Constructing a Z-buffer of the image's width and height
2. Reading packed vectors of indices and vertices from input file
3. Assembling triangles from the packed vectors
4. Determining a bounding box for each triangle
5. Iterating over each pixel in the bounding box
6. Calculating barycentric coordinates to determine if the pixel is inside the triangle
7. Using barycentric coordinates to calculate relative depth
8. Updating the Z-buffer with closest depth
9. Using the contents of the completed Z-buffer to create a pixels byte array
10. Writing the pixels byte array to output file

Usage: 
`
rusterizer Meshfile Imagefile image_width image_height [-w | --wireframe]
`


Supported Mesh files: .obj

Supported Image files: .png

Currently a work in progress, although most components are considered complete at this point.
