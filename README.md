# __Fancyndex__ #

## *A fancy web index to list your favorites directories & files.* ##
__Built with [CppCMS](http://cppcms.com/wikipp/en/page/main) & [Boost/Filesystem library](http://www.boost.org/doc/libs/1_65_0/libs/filesystem/doc/index.htm).__

### Dependencies :
#### ArchLinux
> AUR Packages, I suggest to use `pacaur` (do the job for Arch non-User Repositories like `pacman`).

```
pacaur -S cppcms
pacaur -S boost
pacaur -S boost-libs
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
>- Web interface (0.2) ;
>- Boostrap (0.2.5) ;
>- Documentation ;
>- Nginx reverse-proxy doc ;
>- Create thumbnails for images & videos ;
>- Create zip archive on the fly ;
>- Database to count the number of downloads.
