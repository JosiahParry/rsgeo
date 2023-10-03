test_that("LineStrings are constructed in order", {
  skip_if_not_installed("sf")
  skip_on_cran()

  x <- 1:10
  y <- 10:1
  id <- x
  line <- geom_linestring(x, y, id)

  crds <- sf::st_coordinates(sf::st_as_sfc(line))

  expect_equal(x, crds[, "X"])
  expect_equal(y, crds[, "Y"])
  expect_equal(id, crds[, "L1"])
})


test_that("Polygons are constructed in order", {
  m <- matrix(c(0, 1, 1, 0, 0, 0, 0, 1, 1, 0), ncol = 2)
  m2 <- m - 0.5
  all_m <- cbind(rbind(m, m2), rep(1:2, each = 5))

  rs_ply <- geom_polygon(all_m[,1], all_m[,2], all_m[,3])

  sf_plys <- sf::st_sfc(
    sf::st_polygon(list(m)),
    sf::st_polygon(list(m2))
  )

  expect_equal(sf::st_as_sfc(rs_ply), sf_plys, ignore_attr = TRUE)
})

