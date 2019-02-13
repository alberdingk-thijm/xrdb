# xcolors

A (tiny) library to read X Resource Manager colours from Rust.

This is a small, WIP and very basic library to abstract away some of the code to read
a user's color definitions from Xlib's [X Resources](https://en.wikipedia.org/wiki/X_resources).

It is designed to be included as part of graphical applications that want to theme their windows according to
a user's X resources.

Obviously, X is needed in order for `xcolors` to work.
