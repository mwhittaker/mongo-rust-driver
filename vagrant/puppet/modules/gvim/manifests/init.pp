class gvim {
    package { "libcanberra-gtk-module":
        ensure => present;
    }

    case $operatingsystem {
        debian, ubuntu: {
            package { "vim-gtk":
                ensure  => present,
                require => Package["libcanberra-gtk-module"];
            }
        }

        centos, redhat, fedora: {
            package { "vim-X11":
                ensure => present,
            }
        }
    }
}
