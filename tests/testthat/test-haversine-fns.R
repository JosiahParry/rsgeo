test_that("multiplication works", {
  x <- sf::st_multipoint(matrix(runif(20, -90, 90), ncol = 2)) |>
    sf::st_sfc() |>
    sf::st_cast("POINT") |>
    from_sfc()

  # check same lenght as x
  haversine_destination(x, 1.1:10.1, 10.1:1.1)

  # check scalars
  haversine_destination(x, 10, 10)

  # check mixed
  haversine_destination(x, 1.1:10.1, 10)
  haversine_destination(x, 10, 1.1:10.1)


  # check intermediates
  # scalar
  haversine_intermediate(x, rev(x), 10)

  # same length
  haversine_intermediate(x, rev(x), 10.1:1.1)
})
