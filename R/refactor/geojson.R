
# fp <- "/Users/josiahparry/github/spatialindex/local_authority_districts.geojson"

#' Read GeoJSON file
#'
#' Read a GeoJson file using the `geojson` crate. This function will crash on
#' geojson files ~ 50mb +. Working on it though....
#'
#' To improve reading speed set `--max-ppsize=500000` when creating an R session.
#'
#' @param filepath the file path to be read
#'
#' @export
read_geojson <- function(filepath) {
  props <- read_geojson_props(filepath)
  geoms <- read_geojson_features(filepath)
  props[["geometry"]] <- structure(
    geoms,
    class = determine_geoms_class(geoms)
  )

  props
}

