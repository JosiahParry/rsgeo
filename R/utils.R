flatten_geoms <- function(x) {
  stopifnot(all(vapply(x, inherits, logical(1), "rsgeo")))
  do.call(`c`, x)
}
