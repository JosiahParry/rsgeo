cast_geoms <- function(x, to) {
  UseMethod("cast_geoms")
}

.cast_geom <- function(x, to) {
  cls <- class(x)
  f <- switch(
    cls,
    "point" = cast_point,
    "multipoint" = cast_multipoint,
    "linestring" = cast_linestring,
    "multilinestring" = cast_multilinestring,
    "polygon" = cast_polygon,
    "multipolygon" = cast_multipolygon
  )

  res <- f(x, to)

  if (is.na(res)) warning("Cannot convert `", cls, "` to `", to, "`", call. = FALSE)
  res
}


# pnts <- geom_points_xy(runif(100), runif(100))
#
# cast_points(pnts, "MULTIPOINT")
