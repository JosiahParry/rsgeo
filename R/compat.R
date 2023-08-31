st_as_sfc.rsgeo <- function(x) {
  bbox <- structure(bounding_box(x), class = "bbox")
  geoms <- to_sfc(x)
  new_class <- c(gsub("rs_", "sfc_", class(x)[1]), "sfc")
  structure(
    geoms,
    class = new_class,
    bbox = bbox,
    crs = sf::st_crs(NA),
    precision = 0
  )
}


wk_handle.rsgeo <- function(handleable, handler, ...) {
  wk::wk_handle(sf::st_as_sfc(handleable), handler, ...)
}

wk_crs.rsgeo <- function(x) NA

#' Plot Geometries
#'
#' @details
#'
#' Plotting geometries utilizes `wk::wk_plot()`. The rust geometries are
#' handled by first converting to an `sfc` object in the `wk::wk_handle()`
#' method thus requiring both packages for plotting.
#'
#' @param x an object of class `rsgeo`
#' @param ... arguments passed to `wk::wk_plot()`
#'
#' @export
#' @returns NULL
#' @examples
#' if (rlang::is_installed(c("sf", "wk"))) {
#'   plot(geom_linestring(1:10, runif(10)))
#' }
plot.rsgeo <- function(x, ...) {
  rlang::check_installed(c("wk", "sf"))
  wk::wk_plot(x, ...)
}
