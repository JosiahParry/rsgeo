test_that("multiplication works", {

   polys <- from_sfc(sfdep::guerry$geometry)
   signed_area(polys)
   unsigned_area(polys)

   geodesic_signed_area(polys)
   geodesic_unsigned_area(polys)

   # test areas
   x <- from_sfc(sfdep::guerry$geometry)

   y <- x[1:10]
   y[c(1, 5, 10)] <- NA


   signed_area(x)
   signed_area(y)
   unsigned_area(x)
   unsigned_area(y)
   signed_area_geodesic(x)
   signed_area_geodesic(y)
   unsigned_area_geodesic(x)
   unsigned_area_geodesic(y)

   geom_point(as.double(1:10), as.double(11:20))


})


