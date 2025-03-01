pkgname=ovpn-status
pkgver=0.4.0
pkgrel=2
pkgdesc='Show OVPN (provider) server status in the terminal'
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgname"

package() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
