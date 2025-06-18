pkgname=ovpn-status
pkgver=0.7.1
pkgrel=5
pkgdesc='Show OVPN (provider) server status in the terminal'
arch=('x86_64')
makedepends=('git' 'cargo' 'clang' 'mold')
url="https://github.com/APT37/$pkgname"
license=('MIT-0')

package() {
    install -Dm755 "$startdir/target/release/$pkgname" -t "$pkgdir/usr/bin"
}
