# 6 geometry types supported
# (Multi) Point, LineString, and Polygon
format_rsgeom <- function(x, width = NULL, ...) {

  if (is.null(x)) return(format(NULL))

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
    msg <- paste0(substr(msg, 1, width - 6), "...")
  }

  formatC(msg)

}

format_rsgeoms <- function(x, width = NULL, ...) {
  vapply(x, format, character(1), width = width)
}


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



# vec_ptype_abbr.vctrs_percent

# vctrs abbreviation  -----------------------------------------------------
#' Internal vet
#' @import vctrs
#' @keywords internal
#' @name vector-compatibility
NULL
#' @export
vec_ptype_abbr.point <- function(x, ...) "point"
#' @export
vec_ptype_abbr.multipoint <- function(x, ...) "mpnt"
#' @export
vec_ptype_abbr.linestring <- function(x, ...) "lnstr"
#' @export
vec_ptype_abbr.multilinestring <- function(x, ...) "mlnst"
#' @export
vec_ptype_abbr.polygon <- function(x, ...) "poly"
#' @export
vec_ptype_abbr.multipolygon <- function(x, ...) "mpoly"
#' @export
vec_ptype_abbr.rs_POINT <- function(x, ...) "POINT"
#' @export
vec_ptype_abbr.rs_MULTIPOINT <- function(x, ...) "MULTIPOINT"
#' @export
vec_ptype_abbr.rs_LINESTRING <- function(x, ...) "LINESTRING"
#' @export
vec_ptype_abbr.rs_MULTILINESTRING <- function(x, ...) "MULTILINESTRING"
#' @export
vec_ptype_abbr.rs_POLYGON <- function(x, ...) "POLYGON"
#' @export
vec_ptype_abbr.rs_MULTIPOLYGON <- function(x, ...) "MULTIPOLYGON"
#' @export
vec_ptype_abbr.rs_GEOMETRYCOLLECTION <- function(x, ...) "GEOMETRYCOLLECTION"








print_rsgeom <- function(x, width = NULL) {
  if (is.null(x)) return(invisible(x))
  msg <- format_rsgeom(x, width)
  cat(msg)
  invisible(x)
}

# vctrs handles this for me
print_rsgeoms <- function(x, n = Inf, width = NULL) {
  for (i in 1:min(length(x), n)) {
    print_rsgeom(x[[i]], width = width)
    cat("\n")
  }
  invisible(x)
}
# individual printing

#' Printing and formatting
#'
#' @rdname print
#' @export
print.point <- print_rsgeom
#' @rdname print
#' @export
print.multipoint <- print_rsgeom
#' @rdname print
#' @export
print.polygon <- print_rsgeom
#' @rdname print
#' @export
print.multipolygon <- print_rsgeom
#' @rdname print
#' @export
print.linestring <- print_rsgeom
#' @rdname print
#' @export
print.multilinestring <- print_rsgeom
#'
# multi printing
#' #' @export
#' print.rs_POINT <- print_rsgeoms
#' #' @export
#' print.rs_MULTIPOINT <- print_rsgeoms
#' # #' @export
#' #print.rs_POLYGON <- print_rsgeoms
#' #' @export
#' print.rs_MULTIPOLYGON <- print_rsgeoms
#' #' @export
#' print.rs_LINESTRING <- print_rsgeoms
#' #' @export
#' print.rs_MULTILINESTRING  <- print_rsgeoms
#' #' @export
#' print.rs_GEOMETRYCOLLECTION  <- print_rsgeoms


