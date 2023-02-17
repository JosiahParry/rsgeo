plot_rsgeom <- function(x, ...) plot(as_sfg(x), ...)
plot_rsgeoms <- function(x, ...) plot(sf::st_as_sfc(x), ...)

# ploting
#' @export
plot.polygon <- function(x, ...) plot_rsgeom(x, ...)

#' @export
plot.multipolygon <- function(x, ...) plot_rsgeom(x, ...)

#' @export
plot.point <- function(x, ...) plot_rsgemo(x, ...)

#' @export
plot.multipoint <- function(x, ...) plot_rsgeom(x, ...)

#' @export
plot.linestring <- function(x, ...) plot_rsgeom(x, ...)

#' @export
plot.multilinestring <- function(x, ...) plot_rsgeom(x, ...)

#' @export
plot.rs_POINT <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_MULTIPOINT <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_LINESTRING <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_MULTILINESTRING <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_POLYGON <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_MULTIPOLYGON <- function(x, ...) plot_rsgeoms(x, ...)

#' @export
plot.rs_GEOMETRYCOLELCTION <- function(x, ...) plot_rsgeoms(x, ...)

