
#' Construct Geometries
#'
#' Constructs geometries from numeric vectors.
#'
#' @param x a vector of x coordinates
#' @param y a vector of y coordinates
#' @param id the feature identifier
#' @param ring the id of the polygon ring
#' @export
#' @rdname construction
#' @examples
#'
#' geom_point(3, 0.14)
#' geom_multipoint(1:10, 10:1)
#' geom_linestring(1:10, 10:1)
#' geom_polygon(c(0, 1, 1, 0, 0), c(0, 0, 1, 1, 0))
geom_multipoint <- function(x, y, id = 1) {
  geom_multipoint_(as.double(x), as.double(y), as.integer(id))
}

#' @export
#' @rdname construction
geom_linestring <- function(x, y, id = 1) {
  geom_linestring_(as.double(x), as.double(y), as.integer(id))
}

#' @export
#' @rdname construction
geom_polygon <- function(x, y, id = 1, ring = 1) {
  geom_polygon_(
    as.double(x),
    as.double(y),
    as.integer(id),
    as.integer(ring)
  )
}

#
#
# # check that default value of 1 works
# set.seed(0)
# x <- rnorm(10)
# set.seed(1)
# y <- rnorm(10)
#
# # expect success
# geom_point(x, y)
#
# # expect error
# geom_point(x[1], y)
#
# # expect success
# geom_point(c(NA_real_, 1), as.double(2:1))
#
# # check that default value of 1 scalar works
# geom_multipoint(x, y)
#
# # check that we get an error when id not 1 or less than x
# geom_multipoint(x, y, 1:2)
#
# # check that x cannot be shorter than y
# geom_multipoint(x[1], y)
# geom_multipoint(x, y[1])
#
# # check that id cannot be longer than x
# geom_multipoint(x, y, 1:11)
#
