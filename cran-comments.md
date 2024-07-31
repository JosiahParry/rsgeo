## R CMD check results

0 errors | 0 warnings | 0 notes

* Addresses CRAN removal:
    - updating libR-sys dependency for MacOS oldrel errors
    - setting minimum version of rustc >= 1.70.0 in `SystemRequirements`. **NOTE** this will result in build errors on fedora due to outdated Rust installation. 

* The large tarball size is due to vendored dependencies. 
