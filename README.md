# __Fancyndex__ #

## *A fancy web index to list your favorites directories & files.* ##

### Dependencies :
#### ArchLinux
> AUR Packages, I suggest to use `pacaur` (do the job for Officials Repositories like `pacman`).

```
pacaur -S libmicrohttpd (community)
pacaur -S yaml-cpp      (community)
pacaur -S boost         (extra)
pacaur -S boost-libs    (extra)
pacaur -S iod-git       (AUR)
pacaur -S silicon-git   (AUR)
```

### Others distribs :
> TODO

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

Documentation will come ...

## __version 0.4__ ##

Done :
- Silicon C++ API served by microhttpd server.

- Web UI :
    * JQuery: OK;
    * AJAX: OK;
    * Bootstrap 4.0: OK;

- User actions :
    * Sort by size;
    * Sort by date;
    * Sort by name;
    * Download a single file;

> TODO :
>- Create & download zip (7z certainly) archive (whole folder or multiples selections) ;
>- Create thumbnails for images & videos (by .extension);
>- Documentation ;
>- Nginx https reverse-proxy doc ;
