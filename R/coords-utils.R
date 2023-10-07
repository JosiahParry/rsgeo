#' @examples
#' lines <- geom_linestring(1:10, 1:10)
#' n_coords(lines)
#' coord_first(lines)
#' coord_last(lines)
#' coord_n(lines, 5)
#' @export
#' @rdname coord_utils
coord_n <- function(x, n) {
  coord_n_(x, as.integer(n))
}
