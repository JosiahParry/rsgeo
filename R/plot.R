plot_rsgeo <- function(x, ...) plot(as_sf(x), ...)

# ploting
#' @export
plot.polygon <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.multipolygon <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.point <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.multipoint <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.linestring <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.multilinestring <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_POINT <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_MULTIPOINT <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_LINESTRING <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_MULTILINESTRING <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_POLYGON <- function(x, ...) plot_rsgeo(x, ...)

#' @export
plot.rs_MULTIPOLYGON <- function(x, ...) plot_rsgeo(x, ...)
