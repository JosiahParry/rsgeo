x <- from_sfc(sfdep::guerry$geometry)
minimum_rotated_rect(x)
extreme_coords(x)
convex_hull(x)
concave_hull(x, 0.5)
bounding_box(x)

y <- x
y[c(1, 5, 10)] <- NA

minimum_rotated_rect(y)
extreme_coords(y)
convex_hull(y)
concave_hull(y, 0.5)
bounding_box(y)
