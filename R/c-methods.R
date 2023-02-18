
ptype_geom <- function(type) {
  type <- tolower(type)
  match.arg(
    type,
    c("point", "multipoint", "polygon",
      "multipolygon", "linestring", "multilinestring")
  )

  # printing this will beak it
  structure(null_pntr(), class = type)
}

ptype_geoms <- function(type) {
  type <- tolower(type)
  match.arg(
    type,
    c("point", "multipoint", "polygon",
      "multipolygon", "linestring", "multilinestring",
      "geometrycollection")
  )

  # make the scalar a point if type is geometrycollection
  scalar_type <- ifelse(type == "geometrycollection", "point", type)

  # printing this will beak it
  structure(
    list(ptype_geom(scalar_type)),
    class = c(paste0("rs_", toupper(type)), "vctrs_vctr", "list")
  )

}


# rs <- tidyr::crossing(x = scalar_types, y = scalar_types) |>
#   dplyr::filter(x != y)
#
# x <- rs$x
# y <- rs$y
#
# glue_template <- '
# vec_ptype2.rs_{{toupper(x)}}.rs_{{toupper(y)}} <- function(x, y, ...) {
#   ptype_geoms("GEOMETRYCOLLECTION")
# }
# '

# lhs to rhs
#' @export
vec_ptype2.rs_LINESTRING.rs_MULTILINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_LINESTRING.rs_MULTIPOINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_LINESTRING.rs_MULTIPOLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_LINESTRING.rs_POINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_LINESTRING.rs_POLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTILINESTRING.rs_LINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTILINESTRING.rs_MULTIPOINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTILINESTRING.rs_MULTIPOLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTILINESTRING.rs_POINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTILINESTRING.rs_POLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOINT.rs_LINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOINT.rs_MULTILINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOINT.rs_MULTIPOLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOINT.rs_POINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOINT.rs_POLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOLYGON.rs_LINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOLYGON.rs_MULTILINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOLYGON.rs_MULTIPOINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOLYGON.rs_POINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_MULTIPOLYGON.rs_POLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POINT.rs_LINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POINT.rs_MULTILINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POINT.rs_MULTIPOINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POINT.rs_MULTIPOLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POINT.rs_POLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POLYGON.rs_LINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POLYGON.rs_MULTILINESTRING <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POLYGON.rs_MULTIPOINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POLYGON.rs_MULTIPOLYGON <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}
#' @export
vec_ptype2.rs_POLYGON.rs_POINT <- function(x, y, ...) {
  ptype_geoms("GEOMETRYCOLLECTION")
}

#
# glue_template <- '
# vec_cast.rs_{{toupper(x)}}.rs_{{toupper(y)}} <- function(x, y, ...) {
#   restore_geoms(x)
# }
# '
#
#
# glue::glue(glue_template, .open = "{{", .close = "}}") |> clipr::write_clip()

#' @export
vec_cast.rs_LINESTRING.rs_MULTILINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_LINESTRING.rs_MULTIPOINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_LINESTRING.rs_MULTIPOLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_LINESTRING.rs_POINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_LINESTRING.rs_POLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTILINESTRING.rs_LINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTILINESTRING.rs_MULTIPOINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTILINESTRING.rs_MULTIPOLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTILINESTRING.rs_POINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTILINESTRING.rs_POLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOINT.rs_LINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOINT.rs_MULTILINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOINT.rs_MULTIPOLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOINT.rs_POINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOINT.rs_POLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOLYGON.rs_LINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOLYGON.rs_MULTILINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOLYGON.rs_MULTIPOINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOLYGON.rs_POINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_MULTIPOLYGON.rs_POLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POINT.rs_LINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POINT.rs_MULTILINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POINT.rs_MULTIPOINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POINT.rs_MULTIPOLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POINT.rs_POLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POLYGON.rs_LINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POLYGON.rs_MULTILINESTRING <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POLYGON.rs_MULTIPOINT <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POLYGON.rs_MULTIPOLYGON <- function(x, y, ...) {
  restore_geoms(x)
}
#' @export
vec_cast.rs_POLYGON.rs_POINT <- function(x, y, ...) {
  restore_geoms(x)
}



scalar_types <- c("point", "multipoint", "polygon",
                  "multipolygon", "linestring", "multilinestring")

# c method for scalar types
c._geom <- function(...) {
  elements <- NextMethod()
  if (!all(vapply(elements, inherits, logical(1), scalar_types))) {
    rlang::abort("All elements must be a `geo-types` primitive")
  }

  restore_geoms(elements)
}

# for vectors. this is atrocious code
c._geoms <- function(...) {
  elems <- rlang::list2(...)
  elems <- c(elems, recursive = TRUE)
  if (!all(vapply(elems, inherits, logical(1), scalar_types))) {
    rlang::abort("All elements must be a `geo-types` primitive")
  }
  restore_geoms(elems)
}


#' @export
c.point <- c._geom
#' @export
c.multipoint <- c._geom
#' @export
c.linestring <- c._geom
#' @export
c.multilinestring <- c._geom
#' @export
c.polygon <- c._geom
#' @export
c.multipolygon <- c._geom

#' @export
c.rs_POINT <- c._geoms
#' @export
c.rs_MULTIPOINT <- c._geoms
#' @export
c.rs_POLYGON <- c._geoms
#' @export
c.rs_MULTIPOLYGON <- c._geoms
#' @export
c.rs_LINESTRING <- c._geoms
#' @export
c.rs_MULTILINESTRING <- c._geoms
#' @export
c.rs_GEOMETRYCOLLECTION <- c._geoms
