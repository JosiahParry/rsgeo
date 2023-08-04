# x <- from_sfc(sfdep::guerry$geometry)
# geo <- sfdep::guerry$geometry
#


coords <- function(x) {

  cls <- class(x)[[1]]
  from <- tolower(substr(cls, 4, nchar(cls)))

  switch(
    from,
    "point" = point_to_coords(x),
    "multipoint" = multipoint_to_coords(x),
    "linestring" = linestring_to_coords(x),
    "multilinestring" = multilinestring_to_coords(x),
    "polygon" = polygon_to_coords(x),
    "multipolygon" = multipolygon_to_coords(x),
    stop("No `coords()` method for provided geometry type")
  )

}
