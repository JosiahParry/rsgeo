# 6 geometry types supported
# (Multi) Point, LineString, and Polygon

format_rsgeom <- function(x, width = NULL, ...) {

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

  formatC(msg)

}

format_rsgeoms <- function(x, width = NULL, ...) {
  vapply(x, format, character(1), width = width)
}

print_rsgeom <- function(x, width = NULL) {

  msg <- format_rsgeom(x, width)
  cat(msg)
  invisible(x)
}


print_rsgeoms <- function(x, width = NULL) {
  for (i in seq_along(x)) {
    print_rsgeom(x[[i]], width = width)
    cat("\n")
  }
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



# formatting --------------------------------------------------------------

#' @export
format.point <- format_rsgeom
#' @export
format.multipoint <- format_rsgeom
#' @export
format.polygon <- format_rsgeom
#' @export
format.multipolygon <- format_rsgeom
#' @export
format.linestring <- format_rsgeom
#' @export
format.multilinestring <- format_rsgeom


# multi printing
#' @export
format.rs_POINT <- format_rsgeoms
#' @export
format.rs_MULTIPOINT <- format_rsgeoms
#' @export
format.rs_POLYGON <- format_rsgeoms
#' @export
format.rs_MULTIPOLYGON <- format_rsgeoms
#' @export
format.rs_LINESTRING <- format_rsgeoms
#' @export
format.rs_MULTILINESTRING  <- format_rsgeoms
#' @export
format.rs_GEOMETRYCOLLECTION <- format_rsgeoms


