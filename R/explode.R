#' Explode Lines
#'
#' Given a LineString or MultiLineString, expand the geometry into each and
#' every component Line.
#'
#' @param x an object of class `rs_LINESTRING` or `rs_MULTILINESTRING`
#' @details
#' A `LineString` is composed of one or more `Line`s. A Line is a connected
#' by a start and end coordinate only.
#'
#' @export
#' @returns an object of class `rs_LINESTRING`
#' @examples
#' x <- geom_linestring(1:10, 10:1)
#' length(x)
#' explode_lines(x)
explode_lines <- function(x) {
  if (rlang::inherits_any(x, "rs_LINESTRING")) {
    res <- explode_linestrings_(x)
  } else if (rlang::inherits_any(x, "rs_MULTILINESTRING")) {
    res <- explode_multilinestrings_(x)
  } else {
    cli::cli_abort("{.arg x} must be of class {.cls rs_LINESTRING} or {.cls rs_MULTILINESTRING")
  }

  res
}
