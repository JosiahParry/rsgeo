#' Extract Coordinates
#'
#' Given an `rsgeo` class object, extract the object's coordinates as a data frame.
#' Empty or missing geometries are ignored.
#'
#' @returns
#' A `data.frame` with columns `x`, `y`. Additional columns are returned based
#' on the geometry type. Additional columns are:
#'
#' - `id`
#' - `line_id`: refers to the `LineString` ID for `rs_LINESTRING`, or the component `LineString` in a `MultiLineString`, or as the ring ID for a `Polygon`.
#' - `multilinestring_id`
#' - `polygon_id`
#' - `multipolygon_id`
#'
#' @export
#' @examples
#' pnt <- geom_point(3, 0.14)
#' mpnt <- geom_multipoint(1:10, 10:1)
#' ln <- geom_linestring(1:10, 10:1)
#' ply <- geom_polygon(c(0, 1, 1, 0, 0), c(0, 0, 1, 1, 0))
#'
#' coords(pnt)
#' coords(mpnt)
#' coords(ln)
#' coords(union_geoms(rep(ln, 2)))
#' coords(ply)
#' coords(union_geoms(rep(ply, 2)))
coords <- function(x) {

  cls <- class(x)[[1]]
  from <- tolower(substr(cls, 4, nchar(cls)))

  switch(
    from,
    "point" = point_to_coords(x),
    "multipoint" = multipoint_to_coords(x),
    "linestring" = linestring_to_coords(x),
    "multilinestring" = multilinestring_to_coords(x),
    "polygon" = polygon_to_coords(x),
    "multipolygon" = multipolygon_to_coords(x),
    stop("No `coords()` method for provided geometry type")
  )

}


