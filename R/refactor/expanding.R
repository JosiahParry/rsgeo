# define helpers
.expand_geom <- function(x) {
  cls <- class(x)

  f <- switch(
    cls,
    "multipoint" = expand_multipoint,
    "linestring" = expand_linestring,
    "multilinestring" = expand_multilinestring,
    "polygon" = expand_polygon,
    "multipolygon" = expand_multipolygon,
    warning("Cannot expand `", cls, call. = FALSE)
  )

  if (!is.function(f)) return(list(NULL))
  res <- f(x)

  if (is.null(res)) warning("Cannot expand `", cls, call. = FALSE)
  res
}


# define helpers
.expand_geoms <- function(x, flat = FALSE) {
  cls <- tolower(class(x)[1])

  f <- switch(
    cls,
    "rs_multipoint" = expand_multipoints,
    "rs_linestring" = expand_linestrings,
    "rs_multilinestring" = expand_multilinestrings,
    "rs_polygon" = expand_polygons,
    "rs_multipolygon" = expand_multipolygons,
    warning("Cannot expand `", cls, call. = FALSE)
  )

  if (!is.function(f)) return(list(NULL))
  res <- f(x)

  if (is.null(res)) warning("Cannot expand `", cls, call. = FALSE)

  if (flat) return(restore_geoms(unlist(res)))
  res
}


#' @export
#' @rdname cast_geoms
expand_geoms <- function(x, ...) {
  UseMethod("expand_geoms")
}

#' @export
expand_geoms.point <- .expand_geom
#' @export
expand_geoms.multipoint <- .expand_geom
#' @export
expand_geoms.polygon <- .expand_geom
#' @export
expand_geoms.multipolygon <- .expand_geom
#' @export
expand_geoms.linestring <- .expand_geom
#' @export
expand_geoms.multilinestring <- .expand_geom
#' @export
expand_geoms.rs_POINT <- .expand_geoms
#' @export
expand_geoms.rs_MULTIPOINT <- .expand_geoms
#' @export
expand_geoms.rs_POLYGON <- .expand_geoms
#' @export
expand_geoms.rs_MULTIPOLYGON <- .expand_geoms
#' @export
expand_geoms.rs_LINESTRING <- .expand_geoms
#' @export
expand_geoms.rs_MULTILINESTRING <- .expand_geoms
#' @export
expand_geoms.rs_GEOMETRYCOLLECTION <- .expand_geoms
