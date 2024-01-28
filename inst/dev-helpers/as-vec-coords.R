
as_vec_coords <- function(crds) {
  coords <- glue::glue("  [ {crds$x}, {crds$y} ]", .sep = ",")
  inner <- glue::glue_collapse(coords, ",\n", last = ",\n")
  res <- paste0("vec![\n", inner, "\n]")
  clipr::write_clip(res)
  cat(res)
  invisible(res)
}
