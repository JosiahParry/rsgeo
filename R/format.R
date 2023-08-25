# geo <- sfdep::guerry$geometry
# x <- from_sfc(geo)

#' @export
format.Geom <- function(x, width = NULL, ...) {

  if (is.null(x)) return(format(NULL))

  if (is.null(width)) width <- options("width")[["width"]]
  msg <- print_geom(x)

  if (nchar(msg) > width) {
    msg <- paste0(substr(msg, 1, width - 6), "...")
  }

  formatC(msg, ...)
}

#' @export
format.rsgeo <- function(x, ...) {
  chrs <- vapply(x, format, character(1), ...)
  chrs[which(chrs == "NULL")] <- NA
  chrs
}

