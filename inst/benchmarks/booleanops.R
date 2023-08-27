library(sf)

grd <- st_make_grid(n = c(500, 300), cellsize = 2, offset = c(0, 0))
grd <- st_make_grid(n = c(100, 100), cellsize = 2, offset = c(0, 0))

x <- from_sfc(grd)
xgeo <- geos::as_geos_geometry(grd)

bench::mark(
  sf = st_intersects(grd, grd),
  rs = intersects_sparse(x, x),
  rs_cache = intersects_sparse_cached(x, x),
  rs_cache2 = intersects_sparse_cached2(x, x),
  geos = geos::geos_intersects_matrix(xgeo, xgeo),
  # times = 5
  check = FALSE
)



n <- length(grd)

pnt <- st_sample(grd, n)
pnt_rs <- from_sfc(pnt)
pnt_g <- geos::as_geos_geometry(pnt)


microbenchmark::microbenchmark(
  sf = st_contains(grd, pnt),
  rs = contains_sparse(x, pnt_rs),
  rs_cached = contains_sparse_cached(x, pnt_rs),
  geos = geos::geos_contains_matrix(xgeo, pnt_g),
  times = 3
)


microbenchmark::microbenchmark(
  sf = st_within(grd, pnt),
  rs = within_sparse(x, pnt_rs),
  rs_cached = within_sparse_cached(x, pnt_rs),
  geos = geos::geos_within_matrix(xgeo, pnt_g),
  times = 10
)
