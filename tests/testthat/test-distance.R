
set.seed(1)
x <- geom_point(runif(10, -1, 1), runif(10, -1, 1))
y <- rev(x)

distance_euclidean_matrix(x, y)
distance_hausdorff_matrix(x, y)
distance_vicenty_matrix(x, y)
distance_geodesic_matrix(x, y)
distance_haversine_matrix(x, y)


distance_euclidean_pairwise(x, y)
distance_hausdorff_pairwise(x, y)
distance_vicenty_pairwise(x, y)
distance_geodesic_pairwise(x, y)
distance_haversine_pairwise(x, y)
