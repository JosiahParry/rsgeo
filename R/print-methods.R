# 6 geometry types supported
# (Multi) Point, LineString, and Polygon

print_rsgeoms <- function(x, width = NULL) {
  for (i in seq_along(x)) {
    print_rsgeom(x[[i]], width = width)
    cat("\n")
  }
  invisible(x)
}

print_rsgeom <- function(x, width = NULL) {

  if (is.null(width)) width <- options("width")[["width"]]
  msg <- capture.output(print_geom(x))
  geom_types <- c(
    "Point",
    "MultiPoint",
    "LineString",
    "MultiLineString",
    "Polygon",
    "MultiPolygon"
  )

  msg <- sub(paste(geom_types, collapse = "|"), "", msg)
  if (nchar(msg) > width) {
    msg <- paste0(substr(msg, 1, width - 6), "....")
  }

  cat(msg)
  invisible(x)
}

# individual printing
#' @export
print.point <- print_rsgeom
#' @export
print.multipoint <- print_rsgeom
#' @export
print.polygon <- print_rsgeom
#' @export
print.multipolygon <- print_rsgeom
#' @export
print.linestring <- print_rsgeom
#' @export
print.multilinestring <- print_rsgeom

# multi printing
#' @export
print.rs_POINT <- print_rsgeoms
#' @export
print.rs_MULTIPOINT <- print_rsgeoms
#' @export
print.rs_POLYGON <- print_rsgeoms
#' @export
print.rs_MULTIPOLYGON <- print_rsgeoms
#' @export
print.rs_LINESTRING <- print_rsgeoms
#' @export
print.rs_MULTILINESTRING  <- print_rsgeoms


