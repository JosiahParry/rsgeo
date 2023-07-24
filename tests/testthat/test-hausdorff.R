rextendr::document()
devtools::load_all()
library(sf)
library(testthat)

hd <- function(x, y) sf::st_distance(x, y, which = "Hausdorff", by_element = TRUE)

geo <- sfdep::guerry$geometry
pnts <- sf::st_centroid(geo)

x <- from_sfc(geo)
y <- from_sfc(pnts)


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
lns <- st_cast(geo[[11]], "LINESTRING") |>
  st_sfc()

rslns <- from_sfc(lns)[[1]]

expect_equal(
  hausdorff_dist(x[[10]], rslns),
  hd(geo[[10]], lns)
)



# Poly to Point
st_distance(geo[[10]], pnts[[1]], which = "Hausdorff")
hausdorff_dist(x[[10]], y[[1]])

# Poly to MultiPolygon
mply <- sf::st_union(geo[1:3])
mplyrs <- from_sfc(mply)[[1]]

expect_equal(
  st_distance(geo[[10]], mply, which = "Hausdorff", by_element = TRUE),
  hausdorff_dist(x[[10]], mplyrs)
)



# Polygon to MultiLineString
mlns <- geo[1:3] |>
  st_cast("MULTILINESTRING") |>
  st_union()

mlnsrs <- from_sfc(mlns)[[1]]

st_distance(geo[[10]], mlns, which = "Hausdorff")

hausdorff_dist(x[[10]], mlnsrs)

# Poly to MultiPoint
mpnts <- st_union(st_cast(mlns, "MULTIPOINT"))
mpntsrs <- from_sfc(mpnts)[[1]]

hausdorff_dist(x[[10]], mpntsrs)
st_distance(geo[[10]], mpnts, which = "Hausdorff")


# LineString --------------------------------------------------------------

# Linestring to point
expect_equal(
  hausdorff_dist(rslns, y[[55]]),
  hd(lns, pnts[[55]])
)

# LineString to LineString
lns2 <- st_cast(geo[22], "MULTILINESTRING") |>
  st_cast("LINESTRING")

lns2rs <- from_sfc(lns2)[[1]]

expect_equal(
  hausdorff_dist(rslns, lns2rs),
  hd(lns, lns2)
)


# LineString to Polygon

expect_equal(
  hausdorff_dist(rslns, x[[25]]),
  hd(lns, geo[[25]])
)


# LineString to MultiPolygon
expect_equal(
  hausdorff_dist(lns2rs, mplyrs),
  hd(lns2, mply)
)

  hausdorff_dist(mplyrs, lns2rs)

