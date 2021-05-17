# klamath.wad

This repository contains all materials and scripts necessary to build klamath.wad: a DOOM II IWAD with the themes of
urban warfare and fast-paced shooting. The following items are necessary to build this WAD:

* make
* A Rust installation with Cargo (Stable is fine)
* blender

Once all of the above components are installed, running "make" builds `klamath.wad` and `klamath.deh` in the "dist" 
folder.

`klamath.wad` should be able to run on any DeHacKeD-compliant DOOM engine. For more information on running, see the
`klamath.txt` file, which should also be distributed with the WAD.

## License

Scripting elements, such as the contents of the `util` folder, are licensed under the Apache License version 2.0. See
the `LICENSE-APACHE` file for more information.

Original creative elements, such as the `.blend` files, DeHacKeD files and the final klamath.wad, are released under 
the Creative Commons Attribution-ShareAlike 4.0 International License. See the `LICENSE-BY-SA-4.0` file for more 
information.

This project contains elements borrowed from the FREEDOOM project. These elements are licensed under the BSD 3-Clause
license. See the Freedoom repository for more information.

This project also contains assets under other, compatible licenses. See `materials/matinfo.yml` for information
regarding those licenses.
