# define helpers
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

  if (is.null(res)) warning("Cannot convert `", cls, "` to `", to, "`", call. = FALSE)
  res
}

.cast_geoms <- function(x, to) {

  to <- tolower(to)
  cls <- tolower(class(x))[[1]]

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

  if (any(is.na(res))) warning("Cannot convert `", cls, "` to `", to, "`", call. = FALSE)

  res
}


#' Cast to another geometry type
#' @export
cast_geoms <- function(x, to) {
  UseMethod("cast_geoms")
}

#' @export
cast_geoms.point <- function(x, to) .cast_geom(x, to)
#' @export
cast_geoms.multipoint <- function(x, to) .cast_geom(x, to)
#' @export
cast_geoms.linestring <- function(x, to) .cast_geom(x, to)
#' @export
cast_geoms.multilinestring <- function(x, to) .cast_geom(x, to)
#' @export
cast_geoms.polygon <- function(x, to) .cast_geom(x, to)
#' @export
cast_geoms.multipolygon <- function(x, to) .cast_geom(x, to)

cast_geoms.rs_POINT <- function(x, to) .cast_geoms(x, to)
#' @export
cast_geoms.rs_MULTIPOINT <- function(x, to) .cast_geoms(x, to)
#' @export
cast_geoms.rs_POLYGON <- function(x, to) .cast_geoms(x, to)
#' @export
cast_geoms.rs_MULTIPOLYGON <- function(x, to) .cast_geoms(x, to)
#' @export
cast_geoms.rs_LINESTRING <- function(x, to) .cast_geoms(x, to)
#' @export
cast_geoms.rs_MULTILINESTRING <- function(x, to) .cast_geoms(x, to)
