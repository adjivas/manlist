MAN-list
========

[![Build Status](https://travis-ci.org/adjivas/manlist.svg?branch=master)](https://travis-ci.org/adjivas/manlist)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

This librairy is a *systen's project* for parse all one'smanuel from **POSIX OS** environement.

#### Environment:

Please, check your **MANPATH**'s variable of environment (```echo $MANPATH```). If this variable is empty, you can resolve this with:

```shell
export MANPATH=$(manpath)
```

#### Example:
```shell
$ cargo run
...
command.names:	      mkdep
command.description:	  construct Makefile dependency list
argument.option:	  	  	-a
argument.comment:	  	  		Append to the output file,
argument.comment:	  	  		so that multiple
argument.option:		    	-f
argument.comment:	    			Write the include file dependencies to
argument.option:	    		-p
argument.comment:	    			Cause
command.names:		    rebase
command.description:		"Changes base address of dylibs and bundles"
argument.option:		    	-low_address
argument.comment:	    			Force the base address for the first image to be
argument.option:	    		-high_address
argument.comment:		    		Force the base address for the last image to be such that when that image is loaded it occupies
argument.comment:		    		memory up to
argument.option:		      -arch
argument.comment:			    	Only rebase the specified architecture.  Other architectures in a universal image are left as is.
...
```

#### Directory-Tree:

```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
\__ src
    |__ bin.rs
    \__ lib.rs
```

# License
*manlist*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/manlist/blob/master/LICENSE).

#### About [Dependencies-Crates](https://crates.io/), thanks goes to:
  * [glob >= "0.2.9"](https://crates.io/crates/glob)
