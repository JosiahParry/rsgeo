#' Segments a LineString into `n` equal length LineStrings
#'
#' Given a LineString, segment it into `n` equal length LineStrings.
#' The `n` LineStrings are provided as a `MultiLineString` which can
#' be expanded using `expand_geoms()` and consequently flattened
#' using `flatten_geoms()` if desired.
#'
#' @param x and object of class `rs_LINESTRING`
#' @param n an integer vector determining the number of equal length LineStrings to create
#' @export
#' @returns
#' A vector of class `rs_MULTILINESTRING`
#' @examples
#' x <- geom_linestring(1:10, runif(10, -1, 1))
#'
#' segs <- line_segmentize(x, 3)
#'
#' flatten_geoms(
#'   expand_geoms(segs)
#' )
line_segmentize <- function(x, n) {
  line_segmentize_(x, as.integer(n))
}
