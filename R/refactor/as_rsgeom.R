#' Convert sf object to a Rust geo_type
#' @param x an sfc or sfg object
#' @export
as_rsgeom <- function(x) UseMethod("as_rsgeom")

# sfc methods
#' @export
as_rsgeom.sfc_POINT <- function(x) geom_points(x)
#' @export
as_rsgeom.sfc_MULTIPOINT <- function(x) geom_multipoints(x)
#' @export
as_rsgeom.sfc_LINESTRING <- function(x) geom_linestrings(x)
#' @export
as_rsgeom.sfc_MULTILINESTRING <- function(x) geom_multilinestrings(x)
#' @export
as_rsgeom.sfc_POLYGON <- function(x) geom_polygons(x)
#' @export
as_rsgeom.sfc_MULTIPOLYGON <- function(x) geom_multipolygons(x)


# sfg methods
#' @export
as_rsgeom.POINT <- function(x) geom_point(x[1], x[2])
#' @export
as_rsgeom.MULTIPOINT <- function(x) geom_multipoint(x)
#' @export
as_rsgeom.LINESTRING <- function(x) geom_linestring(x)
#' @export
as_rsgeom.MULTILINESTRING <- function(x) geom_multilinestring(x)
#' @export
as_rsgeom.POLYGON <- function(x) geom_polygon(x)
#' @export
as_rsgeom.MULTIPOLYGON <- function(x) geom_multipolygon(x)

