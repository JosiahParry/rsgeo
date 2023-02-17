
# Custom tibble printing method
format_pillar_value <- function(x, width = NULL, ...) {
  msg <- capture.output(print_geom(x))
  geom_types <- c(
    "Point",
    "MultiPoint",
    "LineString",
    "MultiLineString",
    "Polygon",
    "MultiPolygon"
  )

  indices <- regexpr(paste(geom_types, collapse = "|"), msg)
  ptype <- substr(msg, 1, attr(indices, "match.length"))
  msg <- pillar::style_subtle(paste0("<", ptype, ">"))
  formatC(msg, ...)

}

format_pillar_col <- function(x, width = NULL,...) {
  vals <- vapply(x, format_pillar_value, character(1), ...)
  pillar::new_pillar_shaft_simple(vals)
}

#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_GEOMETRYCOLLECTION <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_POINT <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_MULTIPOINT <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_POLYGON <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_MULTIPOLYGON <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_LINESTRING <- format_pillar_col
#' @importFrom pillar pillar_shaft
#' @export
pillar_shaft.rs_MULTILINESTRING <- format_pillar_col
