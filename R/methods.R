`[.rs_POINT` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_POINT")
}

#' @export
`[.rs_MULTIPOINT` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_MULTIPOINT")
}

#' @export
`[.rs_LINESTRING` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_LINESTRING")
}

#' @export
`[.rs_MULTILINESTRING` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_MULTILINESTRING")
}

#' @export
`[.rs_POLYGON` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_POLYGON")
}

#' @export
`[.rs_MULTIPOLYGON` <- function(x, i) {
  x <- NextMethod()
  structure(x, class = "rs_MULTIPOLYGON")
}



