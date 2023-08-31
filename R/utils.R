#' Flatten a list of rsgeo vectors
#'
#' @param x an object of class `rsgeo`
#' @export
#' @examples
#' pnts <- replicate(
#'   10,
#'   geom_point(runif(1), runif(1)),
#'   simplify = FALSE
#' )
#'
#' flatten_geoms(pnts)
#'
flatten_geoms <- function(x) {
  stopifnot(all(vapply(x, inherits, logical(1), "rsgeo")))
  do.call(`c`, x)
}


#' Convert to an `rsgeo` vector
#'
#' Given an vector of geometries, cast it as an `rsgeo` class object.
#'
#' @param x a geometry vector
#'
#' @export
#' @returns an object of class `rsgeo`
#' @examplesIf rlang::is_installed("sf")
#' x <- sf::st_sfc(sf::st_point(c(0,0)))
#' as_rsgeo(x)
as_rsgeo <- function(x) UseMethod("as_rsgeo")

#' @export
as_rsgeo.default <- function(x) {
  rlang::check_installed("sf")
  as_rsgeo(sf::st_as_sfc(x))
}

#' @export
as_rsgeo.sfc <- function(x) from_sfc(x)
