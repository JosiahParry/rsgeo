#' Combine geometries
#'
#' Given a vector of geometries combine them into a single geometry.
#'
#' @details
#'
#' ### `combine_geoms()`
#'
#' `combine_geoms()` combines a vector of geometries into a vector of lenght one
#' their `MULTI` counterpart.
#'
#' - `rs_POINT` and `rs_MULTIPOINT` -> `rs_MULTIPOINT`
#' - `rs_LINESTRING` and `rs_MULTILINESTRING` -> `rs_MULTILINESTRING`
#' - `rs_POLYGON` and `rs_MULTIPOLYGON` -> `rs_MULTIPOLYGON`
#' - `rs_GEOMETRYCOLLECTION` is not supported
#'
#' ### `union_geoms()`
#' `union_geoms()` creates a union of all geometries
#'
#' - `rs_POINT` - combines and removes repeated points
#' - `rs_MULTIPOINT` - combines and does not remove repeated points
#' - `rs_LINESTRING` - combines and  removes duplicated points
#' - `rs_MULTILINESTRING` - combines and removes duplicated points
#' - `rs_POLYGON` - unions geometries
#' - `rs_MULTIPOLYGON` - unions geometries
#'
#' @param x a vector of geometries
#' @export
combine_geoms <- function(x) {
  cls <- tolower(class(x)[1])
  cls <- substr(cls, 4, nchar(cls))

  res <- switch(
    cls,
    "point" = combine_points(x),
    "multipoint" = combine_multipoints(x),
    "linestring" = combine_linestrings(x),
    "multilinestring" = combine_multilinestrings(x),
    "polygon" = combine_polygons(x),
    "multipolygon" = combine_multipolygons(x)
  )

  restore_geoms(list(res))
}
#
# library(ggplot2)
# guerry |>
#   mutate(geoms = as_rsgeom(geometry)) |>
#   group_by(region) |>
#   summarise(geoms = st_as_sfc(union_geoms(geoms))) |>
#   ggplot() +
#   geom_sf()
#
# x <- as_rsgeom(guerry$geometry)
#
# union_geoms(x[1:3])
#
# union_geoms(geom_points_matrix(matrix(runif(100), ncol = 2)))
