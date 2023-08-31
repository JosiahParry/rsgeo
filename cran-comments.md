## R CMD check results

0 errors | 0 warnings | 2 notes

* This is a new release.
* installed size is  7.0Mb

### Notes

- This is a Rust based package. Follows the requirements in [Using Rust in CRAN Packages](https://cran.r-project.org/web/packages/using_rust.html)
  - dependencies are vendored
  - uses 2 threads with `-J 2` to build dependencies
  - builds entirely offline using `--offline` flag
  - prints the version of `cargo` and `rustc` in the installation log
- The installed size is slightly bigger due to the vendoring of dependencies
  
  
