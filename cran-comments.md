## R CMD check results

0 errors | 0 warnings | 1 notes

* installed size is  7.0Mb

### Notes

This release addresses comments regarding:

- `cargo` being available at `~/.cargo/bin` and not compiling in the Makevars
- `/.cargo/.package-cache` being modified 

As issues were reported with Fedora, I have used Fedora to validate 
the package. To reproduce: 

1. Pull and run `rhub/fedora` Docker image

```
docker pull rhub/fedora
docker run -it rhub/fedora bash
```

2. Install R, git, and cargo. 

```
dnf install R
dnf install git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Note: do not reload environment ariables to ensure that cargo is not on the PATH.

3. Check that `~/.cargo/.package-cache` does not exist

```
~/.cargo/.package-cache
# sh: /root/.cargo/.package-cache: No such file or directory
```

3. Clone the repository (same as submission)
4. Install package using `R CMD INSTALL`

```
mkdir github
cd github
git clone rsgeo
cd rsgeo
chmod +x configure
R CMD INSTALL .
```

5. Confirm that `~/.cargo/.package-cache` does not exist

```
~/.cargo/.package-cache
# sh: /root/.cargo/.package-cache: No such file or directory
```
