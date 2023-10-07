# rust-raylib

Unsafe bindings for [`raylib`](www.raylib.com) and safe wrappers for them.
Currently targets raylib 4.5.

Some features (like logging and text formatting) are excluded from the wrappers,
because there are more safe and idiomatic solutions available for them.

The minor version of the crate follows raylib's major and minor versions (i.e. 0.45.X for raylib 4.5)
It will only be increased when updated to a new raylib version.
The patch version is increased when new fixes and/or improvements are introduced. They may contain breaking changes!
