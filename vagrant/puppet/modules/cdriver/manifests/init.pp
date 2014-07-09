class cdriver {
    vcsrepo { "/home/vagrant/mongo-c-driver":
        ensure   => "present",
        provider => git,
        user     => "vagrant",
        source   => "git://github.com/mongodb/mongo-c-driver"
    }

    $packages = [ "gcc", "automake", "autoconf", "libtool", "pkg-config" ]
    package { $packages:
        ensure => "installed";
    }
    
    exec { "autogen-cdriver":
        command => "/home/vagrant/mongo-c-driver/autogen.sh",
        require => Vcsrepo["/home/vagrant/mongo-c-driver"],
        cwd     => "/home/vagrant/mongo-c-driver",
        user    => "vagrant";
    }

    exec { "make-cdriver":
        command => "/usr/bin/make",
        cwd     => "/home/vagrant/mongo-c-driver",
        require => Exec["autogen-cdriver"],
        user    => "vagrant";
    }

    exec { "make-install-cdriver":
        command => "/usr/bin/make install",
        cwd     => "/home/vagrant/mongo-c-driver",
        require => Exec["make-cdriver"];
    }
}
