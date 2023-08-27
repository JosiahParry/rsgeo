.onLoad <- function(libname, pkgname) {
  vctrs::s3_register("sf::st_as_sfc", "rsgeo")
  vctrs::s3_register("wk::wk_handle", "rsgeo")
  vctrs::s3_register("wk::wk_crs", "rsgeo")
}
