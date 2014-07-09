class vim {
    package { "vim":
        ensure => installed; 
    }

    file { '/home/vagrant/.vimrc':
        owner  => 'vagrant',
        group  => 'vagrant',
        mode   => '0644',
        source => 'puppet:///modules/vim/vimrc';
    }

    $vim_dirs = [ "/home/vagrant/.vim", "/home/vagrant/.vim/bundle" ]

    file { $vim_dirs:
        ensure => "directory",
        owner  => "vagrant",
        group  => "vagrant",
        mode   => "0644";
    }

    exec { "clone-vundle":
        command => "/usr/bin/git clone https://github.com/gmarik/Vundle.vim.git /home/vagrant/.vim/bundle/Vundle.vim",
        creates => "/home/vagrant/.vim/bundle/Vundle.vim",
        require => File["/home/vagrant/.vim/bundle"],
        user    => "vagrant",
    }
   
    file { 'viminfo':
        path   => '/home/vagrant/.viminfo',
        owner  => "vagrant",
        group  => "vagrant",
        ensure => present,
        mode   => 0644;
    }

    exec { "BundleInstall":
        command     => "/usr/bin/vim +PluginInstall +qall",
        environment => ["HOME=/home/vagrant"],
        require     => [
                     Package["vim"], 
                     File["/home/vagrant/.vimrc"],
                     File["/home/vagrant/.viminfo"],
                     Exec["clone-vundle"]
                   ],
        user    => "vagrant",
        group   => "vagrant";
    }
}
