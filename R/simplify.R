#' Simplify Geometry
#'
#' Simplifies LineStrings, Polygons, and their Multi- counterparts.
#'
#' Simplify functions use the Ramer–Douglas–Peucker algorithm. Functions with `vw` use
#' the Visvalingam-Whyatt algorithm.
#'
#' For more see [`geo` docs](https://docs.rs/geo/latest/geo/index.html#simplification).
#' @param x an object of class of `rsgeo`
#' @param epsilon a tolerance parameter. Cannot be equal to or less than 0.
#' @export
#' @rdname simplify
simplify_geoms <- function(x, epsilon) simplify_geoms_(x, as.double(epsilon))

#' @export
#' @rdname simplify
simplify_vw_geoms <- function(x, epsilon) simplify_vw_geoms_(x, as.double(epsilon))

#' @export
#' @rdname simplify
simplify_vw_preserve_geoms <- function(x, epsilon) simplify_vw_preserve_geoms_(x, as.double(epsilon))
