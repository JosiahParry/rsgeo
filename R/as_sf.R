#' Convert geo-type to sfg or sfc object
#' @param x a rust geo-type
#' @export
as_sf <- function(x) UseMethod("as_sf")

# to sfg objects
#' @export
as_sf.point <- function(x) sf::st_point(geom_to_r(x))

#' @export
as_sf.multipoint <- function(x) sf::st_multipoint(geom_to_r(x))

#' @export
as_sf.linestring <- function(x) sf::st_linestring(geom_to_r(x))

#' @export
as_sf.polygon <- function(x) sf::st_polygon(geom_to_r(x))

#' @export
as_sf.multipolygon <- function(x) sf::st_multipolygon(geom_to_r(x))


# to sfc objects
#' @export
as_sf.rs_POINT <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_point))
}

#' @export
as_sf.rs_MULTIPOINT <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_multipoint))
}

#' @export
as_sf.rs_POLYGON <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_polygon))
}

#' @export
as_sf.rs_MULTIPOLYGON <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_multipolygon))
}

#' @export
as_sf.rs_LINESTRING <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_linestring))
}

#' @export
as_sf.rs_MULTILINESTRING <- function(x) {
  x <- geoms_to_r(x)
  sf::st_sfc(lapply(x, sf::st_multilinestring))
}
