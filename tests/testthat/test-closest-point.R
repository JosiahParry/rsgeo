

x <- geom_linestring(1:100, runif(100, 0, 90), rep.int(1:10, 10))
y <- geom_point(runif(10, 0, 90), rnorm(10, 1, 90))

closest_point(x, y)
closest_point_haversine(x, y)
