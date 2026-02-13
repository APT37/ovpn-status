#!/usr/bin/nu

http get https://www.ovpn.com/v2/api/client/entry
| get datacenters | sort
| each { |dc|
    print -n $"(ansi green)($dc.slug | str capitalize)(ansi reset):"
    (http get $"https://status.ovpn.com/datacenters/($dc.slug)/servers" | get data | each {
        |s| print -n $" (if $s.online {ansi green} else {ansi red})($s.name | str substring 3..)(ansi reset)"
    })
    print ""
}