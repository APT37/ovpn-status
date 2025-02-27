pkgname=ovpn-status
pkgver=0.3.2
pkgrel=4
pkgdesc='Show OVPN (provider) server status in the terminal'
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgname"

package() {
  install -Dm755 "$startdir/target/release/ovpn_status" "$pkgdir/usr/bin/$pkgname"
}
