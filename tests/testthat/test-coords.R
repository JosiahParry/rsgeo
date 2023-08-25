

pnt <- geom_point(3, 0.14)
mpnt <- geom_multipoint(1:10, 10:1)
ln <- geom_linestring(1:10, 10:1)
ply <- geom_polygon(c(0, 1, 1, 0, 0), c(0, 0, 1, 1, 0))

coords(pnt)
coords(mpnt)
coords(ln)
coords(union_geoms(rep(ln, 2)))
coords(ply)
coords(union_geoms(rep(ply, 2)))


