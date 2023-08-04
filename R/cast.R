# # Create Geometries -------------------------------------------------------
# geo <- sfdep::guerry$geometry
# pnts <- sf::st_centroid(geo)
# x <- from_sfc(geo)
# y <- from_sfc(pnts)

#' @export
cast_geoms <- function(x, to) {

  stopifnot(
    "`to` must be length 1" = length(to) == 1,
    "Invalid `to` value"= tolower(to) %in% geom_types,
    "Must be an `rsgeo` vector" = inherits(x, "rsgeo")
  )

  cls <- class(x)[[1]]

  from <- tolower(substr(cls, 4, nchar(cls)))

  switch(
    from,
    "point" = cast_points(x, to),
    "multipoint" = cast_multipoints(x, to),
    "linestring" = cast_linestring(x, to),
    "multilinestring" = cast_multilinestrings(x, to),
    "polygon" = cast_polygon(x, to),
    "multipolygon" = cast_multipolygons(x, to),
    stop("No casting method for provided geometry type")
  )

}

# constants
rs_classes <- c(
  "rs_POINT",
  "rs_MULTIPOINT",
  "rs_POLYGON",
  "rs_MULTIPOLYGON",
  "rs_LINESTRING",
  "rs_MULTILINESTRING",
  "rs_GEOMETRY",
  "rs_GEOMETRYCOLLECTION"
)


geom_types <- tolower(substr(rs_classes, 4, nchar(rs_classes)))
