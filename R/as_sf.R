#' Convert geo-type to sfg or sfc object
#' @param x a rust geo-type
#' @export
as_sfg <- function(x) UseMethod("as_sfg")

# to sfg objects
#' @export
as_sfg.point <- function(x) structure(
  geom_to_r(x),
  class = c("XY", "POINT", "sfg")
  )

#' @export
as_sfg.multipoint <- function(x) {
  res <- geom_to_r(x)

  structure(
    res,
    dim = dim(res),
    class = c("XY", "MULTIPOINT", "sfg")
  )

}

#' @export
as_sfg.linestring <- function(x) {
  res <- geom_to_r(x)

  structure(
    res,
    dim = dim(res),
    class = c("XY", "LINESTRING", "sfg")
  )

}

#' @export
as_sfg.polygon <- function(x) {
  structure(
    geom_to_r(x),
    class = c("XY", "POLYGON", "sfg")
  )
}

#' @export
as_sfg.multipolygon <- function(x) {
  structure(
    geom_to_r(x),
    class = c("XY", "MULTIPOLYGON", "sfg")
  )
}


# to sfc objects
st_as_sfc.rs_POINT <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}

st_as_sfc.rs_MULTIPOINT <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}

st_as_sfc.rs_POLYGON <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}


st_as_sfc.rs_MULTIPOLYGON <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}


st_as_sfc.rs_LINESTRING <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}

st_as_sfc.rs_MULTILINESTRING <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}


st_as_sfc.rs_GEOMETRYCOLLECTION <- function(x) {
  sf::st_sfc(lapply(x, as_sfg))
}
