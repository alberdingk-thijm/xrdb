# xrdb

A (tiny) library to read X Resource Manager values from Rust.

This is a small, WIP and very basic library to abstract away some of the code to read
a user's [X Resources](https://en.wikipedia.org/wiki/X_resources).

It is designed to be included as part of graphical applications that want to theme their windows according to
a user's X resources.

Obviously, X is needed in order for `xrdb` to work.
