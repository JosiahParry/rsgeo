
#' @export
bounding_box <- function(x) {
  inherits(x, c(scalar_types, rs_classes))
  unlist(bounding_box_(combine_geoms(c(x))[[1]]))
}
