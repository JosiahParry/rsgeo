

register_method <- function(f) {
  rs_classes <- c(
    "rs_POINT", "rs_MULTIPOINT", "rs_POLYGON", "rs_MULTIPOLYGON", "rs_LINESTRING", "rs_MULTILINESTRING")

  for (cls in rs_classes) {
    vctrs::s3_register(f, cls)
  }
}


# Assigns a single function to each of the rs geo-types classes as a
# method for external package
# for example `assign_methods("sf", "st_precision", rs_prec)`
# assigns the `rs_prec()` function as the method for `sf::st_precision()`
# for all vector geometry types
assign_methods <- function(pkg, f, fx) {
  rs_classes <- c("rs_POINT", "rs_MULTIPOINT", "rs_POLYGON", "rs_MULTIPOLYGON", "rs_LINESTRING", "rs_MULTILINESTRING")

  for (cls in rs_classes) {
    assign(paste0(f, ".", cls), fx, parent.env(environment()))
    vctrs::s3_register(paste0(pkg, "::", f), cls)
  }

}

.onLoad <- function(libname, pkgname) {
  # Register S3 methods for suggests
  # vctrs::s3_register("sf::st_as_sfc", "rs_POLYGON")
  # vctrs::s3_register("sf::st_as_sfc", "rs_MULTIPOLYGON")
  # vctrs::s3_register("sf::st_as_sfc", "rs_POINT")
  # vctrs::s3_register("sf::st_as_sfc", "rs_MULTIPOINT")
  # vctrs::s3_register("sf::st_as_sfc", "rs_LINESTRING")
  # vctrs::s3_register("sf::st_as_sfc", "rs_MULTILINESTRING")
  # vctrs::s3_register("sf::st_as_sfc", "rs_GEOMETRYCOLLECTION")
  # vctrs::s3_register("sf::is_geometry", "rs_POINT")
  # vctrs::s3_register("sf::is_geometry", "rs_MULTIPOINT")
  # vctrs::s3_register("sf::is_geometry", "rs_POLYGON")
  # vctrs::s3_register("sf::is_geometry", "rs_MULTIPOLYGON")
  # vctrs::s3_register("sf::is_geometry", "rs_LINESTRING")
  # vctrs::s3_register("sf::is_geometry", "rs_MULTILINESTRING")
  # vctrs::s3_register("sf::st_bbox", "rs_POINT")
  # vctrs::s3_register("sf::st_bbox", "rs_MULTIPOINT")
  # vctrs::s3_register("sf::st_bbox", "rs_POLYGON")
  # vctrs::s3_register("sf::st_bbox", "rs_MULTIPOLYGON")
  # vctrs::s3_register("sf::st_bbox", "rs_LINESTRING")
  # vctrs::s3_register("sf::st_bbox", "rs_MULTILINESTRING")
  register_method("sf::st_as_sfc")
  register_method("sf::is_geometry")
  register_method("sf::st_bbox")
  assign_methods("sf", "st_precision", rs_prec)
  assign_methods("sdf", "bounding_box", bounding_box)
  assign_methods("sdf", "union_geometry", union_geoms)
  assign_methods("sdf", "combine_geometry", combine_geoms)
  #vctrs::s3_register("dplyr::dplyr_col_select", "sdf")
}
