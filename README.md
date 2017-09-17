# __Fancyndex__ #

## *A fancy web index to list your favorites directories & files.* ##

### Dependencies :
#### ArchLinux
> AUR Packages, I suggest to use `pacaur` (do the job for Officials Repositories like `pacman`).

```
pacaur -S libmicrohttpd (community)
pacaur -S iod-git (AUR)
pacaur -S silicon-git (AUR)
pacaur -S boost (extra)
pacaur -S boost-libs (extra)
pacaur -S iperf3 (community)
```

### How to build ?
> _You will need CMake (3.9) here._

```
mkdir build && cd build
cmake ..
make
```

### How to run ?
```
cd build
make run
```

Let's visit http://localhost:9099
[ ... ]
Documentation will come

## __version 0.1__ ##

Done :
- List directory :
    * Get files' names ;
    * Get directories' names ;
    * Get files' extensions ;
    * Get last write date of files ;
    * Get last write date of directories ;
    * Get files' size (Byte) ;
    * Get directories' size (Byte).


> TODO :
>- C++ API server (0.1.2) ;
>- Bootstrap (0.1.1) ;
>- Web interface - JQuery & AJAX (0.1.3) ;
>- Documentation ;
>- Nginx reverse-proxy doc ;
>- Create thumbnails for images & videos ;
>- Create zip archive on the fly ;
>- Database to count the number of downloads.



symbolic link works only with real construction path
i.e. :

ln -s /home/user/directory /home/user/example/
symlink of /home/user/example/directory --> /home/user/directory


cd /home/user/example
ln -s ../directory /home/user/example/
symlink of /home/user/example/directory --> ../directory/
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------ NOT WORKING
