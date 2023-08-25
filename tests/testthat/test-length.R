set.seed(0)
y <- runif(25, -5, 5)
x <- 1:25

ln <- geom_linestring(x, y)

length_euclidean(ln)
length_geodesic(ln)
length_vincenty(ln)
length_haversine(ln)

ln2 <- ln
ln2[1] <- NA

length_euclidean(ln2)
length_geodesic(ln2)
length_vincenty(ln2)
length_haversine(ln2)

