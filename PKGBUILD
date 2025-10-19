
# Maintainer: terra-fx-t-byte on github

pkgname=aurfromgit
pkgver=1.0.1
pkgrel=1
pkgdesc="A rust based CLI tool, mainly purposed to replace yay if AUR doesnt work by cloning packages directly from GitHub"
arch=('x86_64')
url="https://github.com/terra-fx-t-byte/aurfromgit"
license=('GPL-3.0')
makedepends=('cargo' 'git')
source=("https://github.com/terra-fx-t-byte/aurfromgit.git#tag=$pkgver")
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$srcdir/$pkgname"
    install -Dm0755 -t "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
