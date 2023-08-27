# constants
rs_classes <- c(
  "rs_POINT",
  "rs_MULTIPOINT",
  "rs_POLYGON",
  "rs_MULTIPOLYGON",
  "rs_LINESTRING",
  "rs_MULTILINESTRING",
  "rs_GEOMETRY",
  "rs_GEOMETRYCOLLECTION"
)

geom_types <- tolower(substr(rs_classes, 4, nchar(rs_classes)))
