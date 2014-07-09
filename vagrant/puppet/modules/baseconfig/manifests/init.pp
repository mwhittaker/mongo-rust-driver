class baseconfig {
    exec { 'apt-get update':
        command => '/usr/bin/apt-get update';
    }

    file { '/home/vagrant/.bashrc':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/bashrc';
    }

    file { '/home/vagrant/.bash_aliases':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/bash_aliases';
    }
    
    file { '/home/vagrant/.tmux.conf':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/tmux.conf';
    }
}
