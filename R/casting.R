cast_geoms <- function(x, to) {
  UseMethod("cast_geoms")
}

.cast_geom <- function(x, to) {
  to <- tolower(to)
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


.cast_geoms <- function(x, to) {

  to <- tolower(to)
  cls <- tolower(class(x))

  f <- switch(
    cls,
    "rs_point" = cast_points,
    "rs_multipoint" = cast_multipoints,
    "rs_linestring" = cast_linestrings,
    "rs_multilinestring" = cast_multilinestrings,
    "rs_polygon" = cast_polygons,
    "rs_multipolygon" = cast_multipolygons
    )

  res <- f(x, to)
}
