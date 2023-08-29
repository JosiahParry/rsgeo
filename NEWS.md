# rsgeo 0.1.4

* vendoring rust dependencies to make rsgeo capable of being published on CRAN

# rsgeo 0.1.3

* adding additional parallelization to geodesic, haversine, and vincenty lengths
* paralellization added to minimum bouunding rect
* line_segmentize() is now parallelized

# rsgeo 0.1.0

* Complete rewrite of rsgeo from the ground up.
* distances matrixes are calculated in parallel using rayon
* removes cloning where possible reducing the overall memory footprint even more


# rsgeo 0.0.0.9000

* Added a `NEWS.md` file to track changes to the package.
