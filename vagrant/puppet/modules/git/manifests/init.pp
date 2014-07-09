class git {
    package { "git":
        ensure => installed;
    }

    package { "gitg":
        ensure => "installed";
    }

    file { '/home/vagrant/.gitconfig':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/git/gitconfig';
    }
}
