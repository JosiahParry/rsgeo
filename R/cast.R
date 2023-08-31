#' Cast geometries to another type
#'
#' @param x an object of class `rsgeo`
#' @param to a character scalar of the target geometry type. Must be one of
#' `"point"`, `"multipoint"`, `"polygon"`, `"multipolygon"`, `"linestring"`,
#' or `"multilinestring"`.
#'
#' @details
#'
#' The below conversions are made available. The left hand column indicates
#' the originating vector class and the right hand column indicates the
#' class that it will can be cast to.
#'
#' Note that correctness of conversions will not be checked or verified. If you
#' cast an `rs_MULTIPOINT` to an `rs_POLYGON`, the validity of the polygon
#' cannot be guaranteed.
#'
#' Conversions from an `rs_POLYGON` into an `rs_LINESTRING` will result in only
#' the exterior ring of the polygon ignoring any interior rings if there are any.
#'
#' | From        | To          |
#' | ----------- | ----------- |
#' | `rs_POINT` | `rs_MULTIPOINT` |
#' | `rs_MULTIPOINT` | `rs_POLYGON`, `rs_MULTIPOLYGON`, `rs_LINESTRING`, `rs_MULTILINESTRING` |
#' | `rs_POLYGON` | `rs_MULTIPOINT`, `rs_MULTIPOLYGON`, `rs_LINESTRING`, `rs_MULTILINESTRING` |
#' | `rs_MULTIPOLYGON` | `rs_MULTIPOINT`, `rs_MULTILINESTRING` |
#' | `rs_LINESTRING` | `rs_MULTIPOINT`, `rs_MULTILINESTRING`, `rs_POLYGON` |
#' | `rs_MULTILINESTRING` | `rs_MULTIPOINT`, `rs_MULTIPOLYGON` |
#'
#' @examples
#' ply <- geom_polygon(c(0, 1, 1, 0, 0), c(0, 0, 1, 1, 0))
#' cast_geoms(ply, "linestring")
#' cast_geoms(ply, "multipoint")
#' @export
#' @returns
#' An object of class `rsgeo`
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
    "linestring" = cast_linestrings(x, to),
    "multilinestring" = cast_multilinestrings(x, to),
    "polygon" = cast_polygons(x, to),
    "multipolygon" = cast_multipolygons(x, to),
    stop("No casting method for provided geometry type")
  )

}
