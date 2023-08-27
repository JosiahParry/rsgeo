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
#' @param x a geometry vector
#' @export
as_rsgeo <- function(x) UseMethod("as_rsgeo")
#' @export
as_rsgeo.sfc <- function(x) from_sfc(x)
