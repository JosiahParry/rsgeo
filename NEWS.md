# rsgeo 0.1.7

* `explode_lines()` will expand an `rs_LINESTRING` or `rs_MULTILINESTRING` into their component segments
* Adds `line_segmentize_haversine()` to segment LineStrings in geographic space.
* Adds `geom_line()` to construct a straight line geometry between two point vectors.
* Adds `coord_first()`, `coord_last()`, `coord_n()`, and `n_coords()` functions for working with coordinates of geometries. 
* Adds `densify_euclidean()` and `densify_haversine()` to densify planar and geographic linear geometries respectively.
* Bug fix: `line_segmentize()` would not always return `n` elements (h/t [@Robinlovelace](https://github.com/Robinlovelace))
* `geom_linestring()`, `geom_polygon()` and `geom_multipoint()` constructors ignored order. This was due to the internal use of a `HashMap`. These have been swapped to a `BTreeMap` which preserves order. Additional tests have been added to compare to`sf`s constructors as validation.

# rsgeo 0.1.6

* Adds `configure` and `configure.win` scripts to ensure the package remains on CRAN
* fixes a bug in `format.rsgeo` where `...` were passed into `vapply()` and not `format()` 

# rsgeo 0.1.5

* Initial CRAN release. This couldn't have been possible without the attention to detail of @eitsupi, and the guidance and help from [@cgmossa](https://github.com/CGMossa/), [@Ilia-Kosenkov](https://github.com/Ilia-Kosenkov), [@sorhwell](https://github.com/sorhawell) and the prior art of [@yutannihilation](https://github.com/yutannihilation) and [@jeroen](https://github.com/jeroen)

# rsgeo 0.1.4

* vendoring rust dependencies to make rsgeo capable of being published on CRAN

# rsgeo 0.1.3

* adding additional parallelization to geodesic, Haversine, and Vincenty lengths
* parallelization added to minimum bounding rect
* line_segmentize() is now parallelized

# rsgeo 0.1.0

* Complete rewrite of rsgeo from the ground up.
* distances matrices are calculated in parallel using rayon
* removes cloning where possible reducing the overall memory footprint even more


# rsgeo 0.0.0.9000

* Added a `NEWS.md` file to track changes to the package.
