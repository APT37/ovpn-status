pkgname=ovpn-status
pkgver=0.7.1
pkgrel=1
pkgdesc='Show OVPN (provider) server status in the terminal'
arch=('x86_64')
url="https://github.com/APT37/$pkgname"
license='MIT-0'

package() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
