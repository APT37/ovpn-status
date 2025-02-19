pkgname=ovpn-status
pkgver=0.3.2
pkgrel=2
pkgdesc='Queries the endpoints used by OVPN's status site and shows all servers' status in the terminal'
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgname"

package() {
  install -Dm755 "$startdir/target/release/ovpn_status" "$pkgdir/usr/bin/$pkgname"
}
