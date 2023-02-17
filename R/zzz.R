.onLoad <- function(...) {
  # Register S3 methods for suggests
  vctrs::s3_register("sf::st_as_sfc", "rs_POLYGON")
  vctrs::s3_register("sf::st_as_sfc", "rs_MULTIPOLYGON")
  vctrs::s3_register("sf::st_as_sfc", "rs_POINT")
  vctrs::s3_register("sf::st_as_sfc", "rs_MULTIPOINT")
  vctrs::s3_register("sf::st_as_sfc", "rs_LINESTRING")
  vctrs::s3_register("sf::st_as_sfc", "rs_MULTILINESTRING")
  vctrs::s3_register("sf::st_as_sfc", "rs_GEOMETRYCOLLECTION")
}
