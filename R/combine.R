#' Combine geometries
#'
#' Given a vector of geometries combine them into a single geometry.
#'
#' @details
#'
#' ### `combine_geoms()`
#'
#' `combine_geoms()` combines a vector of geometries into a vector of length one
#' their `MULTI` counterpart.
#'
#' - `rs_POINT` and `rs_MULTIPOINT` -> `rs_MULTIPOINT`
#' - `rs_LINESTRING` and `rs_MULTILINESTRING` -> `rs_MULTILINESTRING`
#' - `rs_POLYGON` and `rs_MULTIPOLYGON` -> `rs_MULTIPOLYGON`
#' - `rs_GEOMETRYCOLLECTION` is not supported
#'
#' ### `union_geoms()`
#'
#' `union_geoms()` creates a union of all geometries removing repeated points
#' or dissolving shared boundaries.
#'
#' - `rs_POINT` - combines and removes repeated points
#' - `rs_MULTIPOINT` - combines removes repeated points
#' - `rs_LINESTRING` - combines and removes duplicated points
#' - `rs_MULTILINESTRING` - combines and removes duplicated points
#' - `rs_POLYGON` - unions geometries into a single geometry
#' - `rs_MULTIPOLYGON` - unions geometries into a single geometry
#'
#' @param x an object of class `rsgeo`
#'
#' @examples
#' pnts <- geom_point(runif(10), runif(10))
#' combine_geoms(pnts)
#'
#' lns <- geom_linestring(1:100, runif(100, -10, 10), rep.int(1:5, 20))
#' union_geoms(lns)
#'
#' x <- c(0, 1, 1, 0, 0)
#' y <- c(0, 0, 1, 1, 0)
#'
#' p1 <- geom_polygon(x, y)
#' p2 <- geom_polygon(x - 1, y + 0.5)
#'
#' z <- c(p1, p2)
#'
#' res <- union_geoms(z)
#' res
#'
#' if (rlang::is_installed(c("sf", "wk"))) {
#'   par(mfrow = c(1, 2))
#'   plot(z)
#'   plot(res)
#' }
#' @export
#' @returns
#' An object of class `rsgeo` of length one.
combine_geoms <- function(x) {
  cls <- tolower(class(x)[1])
  cls <- substr(cls, 4, nchar(cls))

  switch(
    cls,
    "point" = combine_points(x),
    "multipoint" = combine_multipoints(x),
    "linestring" = combine_linestrings(x),
    "multilinestring" = combine_multilinestrings(x),
    "polygon" = combine_polygons(x),
    "multipolygon" = combine_multipolygons(x)
  )
}
