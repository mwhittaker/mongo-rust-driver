# create a new run stage to ensure certain modules are included first
stage { 'pre':
  before => Stage['main']
}

# add the baseconfig module to the new 'pre' run stage
class { 'baseconfig':
  stage => 'pre'
}

include baseconfig
include vim
include gvim
include git
include rust
include tmux
include cdriver
include valgrind
include gpp
