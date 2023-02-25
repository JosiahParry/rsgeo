#' @export
is_geometry <- function(x) UseMethod("is_geometry")
#' @export
is_geometry.default <- function(x) inherits(x, c(scalar_types, rs_classes))
is_geometry.rs_POINT <- function(x) inherits(x, "rs_POINT")
is_geometry.rs_MULTIPOINT <- function(x) inherits(x, "rs_MULTIPOINT")
is_geometry.rs_POLYGON <- function(x) inherits(x, "rs_POLYGON")
is_geometry.rs_MULTIPOLYGON <- function(x)  inherits(x, "rs_MULTIPOLYGON")
is_geometry.rs_LINESTRING <- function(x) inherits(x, "rs_LINESTRING")
is_geometry.rs_MULTILINESTRING <- function(x)  inherits(x, "rs_MULTILINESTRING")



# rs_POINT
# rs_MULTIPOINT
# rs_POLYGON
# rs_MULTIPOLYGON
# rs_LINESTRING
# rs_MULTILINESTRING

guerry <- sfdep::guerry
# m <- matrix(runif(200, -180, 180), ncol = 2)
# x <- geom_points_matrix(m)
# bounding_box(x)

# bounding box convenience function
rs_bbox <- function(x) {
  multi_geom <- combine_geoms(x)[[1]]
  bbox <- bounding_box(multi_geom)
  sf::st_bbox(
    c(
      xmin = bbox[["x_min"]],
      xmax = bbox[["x_max"]],
      ymin = bbox[["y_min"]],
      ymax = bbox[["y_max"]]
    ),
    crs = NA
  )
}

st_bbox.rs_POINT <- function(x) rs_bbox(x)
st_bbox.rs_MULTIPOINT <- function(x) rs_bbox(x)
st_bbox.rs_POLYGON <- function(x) rs_bbox(x)
st_bbox.rs_MULTIPOLYGON <- function(x) rs_bbox(x)
st_bbox.rs_LINESTRING <- function(x) rs_bbox(x)
st_bbox.rs_MULTILINESTRING <- function(x) rs_bbox(x)


rs_prec <- function(x)  attr(x, "precision")

# f is the target function
# fx is the function to be used


