rextendr::document()
devtools::load_all()
library(sf)
library(testthat)


# Create Geometries -------------------------------------------------------
geo <- sfdep::guerry$geometry
pnts <- sf::st_centroid(geo)

x <- from_sfc(geo)
y <- from_sfc(pnts)

lns <- st_cast(geo[[11]], "LINESTRING") |>
  st_sfc()

rslns <- from_sfc(lns)[[1]]

mply <- sf::st_union(geo[1:3])
mplyrs <- from_sfc(mply)[[1]]


mlns <- geo[1:3] |>
  st_cast("MULTILINESTRING") |>
  st_union()

mlnsrs <- from_sfc(mlns)[[1]]

mlns2 <- geo[5:6] |>
  st_cast("MULTILINESTRING") |>
  st_union()

mlns2rs <- from_sfc(mlns2)[[1]]

lns2 <- st_cast(geo[22], "MULTILINESTRING") |>
  st_cast("LINESTRING")

lns2rs <- from_sfc(lns2)[[1]]

# multi points
mpnts <- st_union(st_cast(mlns, "MULTIPOINT"))
mpntsrs <- from_sfc(mpnts)[[1]]

mpnt2 <-  geo[3:7] |>
  st_cast("MULTIPOINT") |>
  st_union()

mpnt2rs <- from_sfc(mpnt2)



# helper function for calculating hausdorff distance using sf / (thus geos)
hd <- function(x, y) sf::st_distance(x, y, which = "Hausdorff", by_element = TRUE)


# Point Geometries --------------------------------------------------------

# Point to Point!!

hausdorff_dist(y[[1]], y[[2]])

pracma::hausdorff_dist(
  matrix(pnts[[1]], ncol = 2),
  matrix(pnts[[2]], ncol = 2)
)

# Point to MultiPoint
coords <- st_coordinates((geo[[1]]))[,1:2]

pracma::hausdorff_dist(
  matrix(pnts[[1]], ncol = 2),
  coords
)

hausdorff_dist(y[[1]], x[[1]])

# MultiPoint --------------------------------------------------------------

# MultiPoint to Point
expect_equal(
  hausdorff_dist(mpntsrs, y[[1]]),
  hd(mpnts, pnts[1])
)

# MultiPoint to MultiPoint
expect_equal(
  hausdorff_dist(mpntsrs, mpnt2rs[[1]]),
  hd(mpnts, mpnt2)
)

# MultiPoint to LineString
expect_equal(
  hausdorff_dist(mpntsrs, rslns),
  hd(mpnts, lns)
)

# MultiPoint to MultiLineString
expect_equal(
  hausdorff_dist(mpntsrs, mlnsrs),
  hd(mpnts, mlns)
)


# MultiPoint to Polygon
expect_equal(
  hausdorff_dist(mpntsrs, x[[70]]),
  hd(mpnts, geo[[70]])
)

# MultiPoint to MultiPolygon
expect_equal(
  hausdorff_dist(mpnt2rs[[1]], mplyrs),
  hd(mpnt2, mply)
)


# LineString --------------------------------------------------------------

# Linestring to point
expect_equal(
  hausdorff_dist(rslns, y[[55]]),
  hd(lns, pnts[[55]])
)

# LineString to MultiPoint
expect_equal(
  hausdorff_dist(rslns, mpntsrs),
  hd(lns, mpnts)
)

# LineString to LineString
expect_equal(
  hausdorff_dist(rslns, lns2rs),
  hd(lns, lns2)
)


# LineString to MultiLineString
expect_equal(
  hausdorff_dist(rslns, mlnsrs),
  hd(lns, mlns)
)


# LineString to Polygon
expect_equal(
  hausdorff_dist(rslns, x[[25]]),
  hd(lns, geo[[25]]),
  tolerance = 0.01
)


# LineString to MultiPolygon
expect_equal(
  hausdorff_dist(lns2rs, mplyrs),
  hd(lns2, mply)
)



# MultiLineString ---------------------------------------------------------

# MultiLinestring to point
expect_equal(
  hausdorff_dist(mlnsrs, y[[55]]),
  hd(mlns, pnts[[55]])
)

# MultiLineString to MultiPoint
expect_equal(
  hausdorff_dist(mlnsrs, mpntsrs),
  hd(mlns, mpnts)
)

# MultiLineString to LineString
expect_equal(
  hausdorff_dist(mlnsrs, lns2rs),
  hd(mlns, lns2)
)

# MultiLineString to MultiLineString
expect_equal(
  hausdorff_dist(mlnsrs, mlns2rs),
  hd(mlns, mlns2)
)


# LineString to Polygon
expect_equal(
  hausdorff_dist(rslns, x[[25]]),
  hd(lns, geo[[25]]),
  tolerance = 0.01
)


# LineString to MultiPolygon
expect_equal(
  hausdorff_dist(lns2rs, mplyrs),
  hd(lns2, mply)
)



# Polygon Methods ---------------------------------------------------------

# Polygon to Polygon
pracma::hausdorff_dist(
  st_coordinates((geo[[1]]))[,1:2],
  st_coordinates((geo[[2]]))[,1:2]
)

expect_equal(
  hausdorff_dist(x[[10]], x[[2]]),
  hd(geo[[10]], geo[[2]])
)


# Polygon to LineString

expect_equal(
  hausdorff_dist(x[[10]], rslns),
  hd(geo[[10]], lns)
)


# Poly to Point
expect_equal(
  hd(geo[[10]], pnts[[1]]),
  hausdorff_dist(x[[10]], y[[1]])
)

# Poly to MultiPolygon
expect_equal(
  st_distance(geo[[10]], mply, which = "Hausdorff", by_element = TRUE),
  hausdorff_dist(x[[10]], mplyrs)
)

# Polygon to MultiLineString
st_distance(geo[[10]], mlns, which = "Hausdorff")

hausdorff_dist(x[[10]], mlnsrs)

# Poly to MultiPoint
hausdorff_dist(x[[10]], mpntsrs)
st_distance(geo[[10]], mpnts, which = "Hausdorff")

