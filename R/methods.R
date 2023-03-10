# `[.rs_POINT` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_POINT")
# }
#
# #' @export
# `[.rs_MULTIPOINT` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_MULTIPOINT")
# }
#
# #' @export
# `[.rs_LINESTRING` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_LINESTRING")
# }
#
# #' @export
# `[.rs_MULTILINESTRING` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_MULTILINESTRING")
# }
#
# #' @export
# `[.rs_POLYGON` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_POLYGON")
# }
#
# #' @export
# `[.rs_MULTIPOLYGON` <- function(x, i) {
#   x <- NextMethod()
#   structure(x, class = "rs_MULTIPOLYGON")
# }
#
#
#
#
# # as.matrix ---------------------------------------------------------------

#' Convert points to matrix
#' @export
as.matrix.rs_POINT <- function(x) do.call(rbind, geoms_to_r(x))
#'
#' #' @export
#' as.matrix.linestring <- function(x) geom_to_r(x)
#'
