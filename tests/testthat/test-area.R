test_that("multiplication works", {

   polys <- from_sfc(sfdep::guerry$geometry)
   signed_area(polys)
   unsigned_area(polys)

   geodesic_signed_area(polys)
   geodesic_unsigned_area(polys)

})


