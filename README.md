
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rsgeo

This package is in very very very very very infantile state. This is an
R package to add bindings to the rust geo crate.

Load the package

``` r
devtools::load_all()
#> ℹ Loading rsgeo
```

Here is a performance challenge i am running into.

Create geometries.

``` r
# get geometry from sf
polys <- sfdep::guerry[["geometry"]] |>
  sf::st_cast("POLYGON")

# cast to rust
rs_polys <- rs_polygons(polys)
```

Benchmark one-to-one intersections.

``` r
bench::mark(
  rust = intersect_poly_poly(rs_polys[[1]], rs_polys[[85]]),
  sf = sf::st_intersects(polys[[1]], polys[[85]]),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 rust         2.99µs   3.32µs   273214.        0B     27.3
#> 2 sf         250.18µs 265.06µs     3713.     358KB     10.4
```

Rust is insanely fast.

Do one-to-many it becomes super slow but still very memory efficient.

``` r
bench::mark(
  rust = intersect_poly_polys(rs_polys[[1]], rs_polys),
  sf = sf::st_intersects(polys[[1]], polys),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression      min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 rust        10.28ms   10.4ms      95.5      512B      0  
#> 2 sf           3.37ms    3.5ms     283.     1.42MB     13.0
```

Rust implementation for `intersect_poly_poly()`. See line 353 in lib.rs.

``` rust
fn intersect_poly_poly(lhs: Robj, rhs: Robj) -> Rbool {
    let xpoly: ExternalPtr<Polygon> = lhs.try_into().unwrap(); 
    let ypoly: ExternalPtr<Polygon> = rhs.try_into().unwrap(); 

    Rbool::from(xpoly.intersects(&*ypoly))

}
```

This is nice and fast.

The implementation for `intersect_poly_polys()`. See line 383

``` rust
fn intersect_poly_polys(lhs: Robj, rhs: List) -> Logicals {
    let n = rhs.len();
    let mut res = Logicals::new(n);
    let xpoly: ExternalPtr<Polygon> = lhs.try_into().unwrap();

    for i in 0..n {
        let ypoly: ExternalPtr<Polygon> = rhs[i].to_owned().try_into().unwrap();
        res.set_elt(i, Rbool::from(xpoly.intersects(&*ypoly)));
    }
    res
}
```
