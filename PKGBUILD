pkgname=ovpn_status
pkgver=0.2.9
pkgrel=1
pkgdesc="Queries the endpoints used by OVPN's status site and shows status information in the terminal"
arch=('any')
url="https://git.nospy.in/Rust/$pkgname"
depends=()
source=()
sha256sums=()

package() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  }