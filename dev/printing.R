#' rextendr::document()
#devtools::load_all()


# POINT -------------------------------------------------------------------

#| point is a single point (similar to sfg)
#| rs_POINT is a list of points (similar to sfc_POINT)

print.point <- function(x) message(capture.output(print_rs_point(x)))

print.rs_POINT <- function(x, n) {
  cat("Rust geo::geometry::Point\n")
  for (i in 1:length(x)) {
    print(x[[i]])
  }
}

# check print methods
set.seed(0)
m <- matrix(runif(10), ncol = 2)
rs_point(10, 100)

rs_points(m)


# LINE --------------------------------------------------------------------

#| I personally dont see an immediate use case for a list of lines
#| but im sure there is
ln <- rs_line(c(0.0, 1.0), c(1, 1))
print_rs_line(ln)


# LineString --------------------------------------------------------------

print.linestring <- function(x) {
  message(capture.output(print_rs_linestring(x)))
}

#| matrix helper

m
lns <- rs_linestring(m)
lns

rs_linestring(m) |>
  linestring_to_points()

library(sf)
library(rsgeo)

print.linestring <- function(x) {
  message(capture.output(print_rs_linestring(x)))
}

set.seed(0)
m <- matrix(runif(10), ncol = 2)
(sf_ln <- sfheaders::sfg_linestring(m))
rs_linestring(sf_ln)


linestrings <- rs_linestrings(lapply(1:10, \(x) matrix(runif(10), ncol = 2)))



# MULTILINESTRING ---------------------------------------------------------
linestrings_to_multilinestring(linestrings) |>
  print_rs_multilinestring()


mlns <- structure(list(structure(c(7.5414596, 7.5416694, 51.9551502, 51.9550808), .Dim = c(2L, 2L)), structure(c(7.5376027, 7.5374521, 51.9570077, 51.9572481), .Dim = c(2L, 2L)), structure(c(7.5361456, 7.5364863, 7.5372242, 7.5373921, 7.5376027, 51.9564927, 51.9565566, 51.9568083, 51.956897, 51.9570077), .Dim = c(5L, 2L)), structure(c(7.5376027, 7.538098, 7.538336, 7.5384622, 7.5385416, 7.5384695, 7.5381901, 51.9570077, 51.9570841, 51.9571428, 51.9571626, 51.9571559, 51.9571833, 51.9573793), .Dim = c(7L, 2L))), class = c("XY", "MULTILINESTRING", "sfg"))

rs_multilinestring(mlns) |>
  print_rs_multilinestring()


# Polygon -----------------------------------------------------------------

polys <- sfdep::guerry$geometry |>
  sf::st_cast("POLYGON")
rs_poly <- rs_polygon(polys[[1]])
rs_polys <- rs_polygons(polys)
geos_polys <- geos::as_geos_geometry(polys)
geos_poly <- geos::as_geos_geometry(polys[1])
x <- rs_polys[[1]]

# not valid but still works
rs_polygon(mlns) |>
  print_rs_polygon()



rs_polygons(polys)


print.polygon <- print_rs_polygon

print.polygon <- function(x, width = options("width")[[1]], ...) {
  msg <- capture.output(print_rs_polygon(x))
  message(paste0(substr(msg, 1, width - 3), "..."))

}






# area --------------------------------------------------------------------

signed_area(rs_polygon(polys[[1]]))

pnts <- sf::st_centroid(polys)





bench::mark(
  geos::geos_area(geos_poly),
  signed_area(x),
  iterations = 100000
)

bench::mark(
  rust = signed_areas(rs_polygons(polys)),
  geos = geos::geos_area(geos::as_geos_geometry(polys)),
  sf = sf::st_area(polys),
  check = FALSE,
  iterations = 5000
)

bench::mark(
  rust = intersect_poly_polys(rs_poly, rs_polys),
  sf = sf::st_intersects(polys[[1]], polys)[[1]],
  check = F
)

bench::mark(
  geos = geos::geos_intersects(geos_polys[[1]], geos_polys),
  rust = intersect_poly_polys(rs_poly, rs_polys),
  check = FALSE
)

bench::mark(
  geos = geos::geos_intersects(geos_polys[[1]], geos_polys[[10]]),
  rust = intersect_poly_poly(rs_poly, rs_polys[[10]])
)
