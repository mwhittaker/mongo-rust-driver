include git

class rust {
    # rust language
    exec { 'wget-rust': 
        command => "/usr/bin/wget http://rust-lang.org/rustup.sh",
        cwd     => "/home/vagrant",
        creates => "/home/vagrant/rustup.sh";
    }
     
    exec { 'chmod-rust':
        command => "/bin/chmod 755 rustup.sh",
        cwd     => "/home/vagrant",
        require => Exec["wget-rust"];
    }
    
    exec { "install-rust":
        command  => "/home/vagrant/rustup.sh",
        cwd      => "/home/vagrant",
        timeout  => 3600,
        require  => Exec["chmod-rust"],
        creates  => "/usr/local/bin/rustc";
    }
      
    # cargo
    vcsrepo { "/home/vagrant/cargo":
        ensure   => present,
        provider => git,
        source   => "git://github.com/rust-lang/cargo.git"
    }

    exec { "submodule-init":
        command => "/usr/bin/git submodule init",
        cwd     => "/home/vagrant/cargo",
        require => Vcsrepo["/home/vagrant/cargo"];
    }

    exec { "submodule-update":
        command => "/usr/bin/git submodule update",
        cwd     => "/home/vagrant/cargo",
        require => Exec["submodule-init"];
    }
    
    exec { "make":
        command => "/usr/bin/make",
        cwd     => "/home/vagrant/cargo",
        require => Exec["submodule-update"];
    }

    exec { "make-install":
        command => "/usr/bin/make install",
        cwd     => "/home/vagrant/cargo",
        require => Exec["make"];
    }

    # raft
    vcsrepo { "/home/vagrant/raft":
        ensure   => present,
        provider => git,
        source   => "git://github.com/mwhittaker/raft.git"
    }
}
