## R CMD check results

0 errors | 0 warnings | 1 notes

* installed size is  7.0Mb

### Notes

This release addresses comments from Dr. Ripley:

> On platforms without a system Rust:
# print cargo and rust versions  
echo `cargo --version` && echo `rustc --version`  
/bin/sh: cargo: command not found  
/bin/sh: rustc: command not found  
Please correct before 2023-09-16 to safely retain your package on CRAN.

This release adds `configure` and `configure.win` to catch this build error. 

  
