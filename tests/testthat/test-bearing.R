# test distances
x <- geom_point(runif(100, 1, 10), rnorm(100, 1, 50))
y <- rev(x)

bearing_geodesic(x, y)
bearing_haversine(x, y)
