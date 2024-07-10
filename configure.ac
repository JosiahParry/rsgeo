AC_INIT

# Export PATH to include Cargo binary directory
export PATH="$PATH:$HOME/.cargo/bin"

# Check for Rustc
AC_CHECK_PROGS([RUSTC], [rustc], [no])
if test "$RUSTC" = "no"; then
    echo "----------------------- [RUST NOT FOUND]---------------------------"
    echo "The 'rustc' compiler was not found on the PATH. Please install Rust"
    echo "from: https://www.rust-lang.org/tools/install"
    echo ""
    echo "Alternatively, you may install Rust from your OS package manager:"
    echo " - Debian/Ubuntu: apt-get install rustc"
    echo " - Fedora/CentOS: dnf install rustc"
    echo " - macOS: brew install rustc"
    echo "-------------------------------------------------------------------"
    echo ""
    exit 1
fi

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "----------------------- [CARGO NOT FOUND]--------------------------"
    echo "The 'cargo' command was not found on the PATH. Please install Cargo"
    echo "from: https://www.rust-lang.org/tools/install"
    echo ""
    echo "Alternatively, you may install Cargo from your OS package manager:"
    echo " - Debian/Ubuntu: apt-get install cargo"
    echo " - Fedora/CentOS: dnf install cargo"
    echo " - macOS: brew install rustc"
    echo "-------------------------------------------------------------------"
    echo ""
    exit 1
fi

AC_OUTPUT