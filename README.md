# clip
A simple, cross platform command line tool for working with the clipboard.

## Supported Operating Systems
clip supports OS X, Linux, and Windows.

## Installing
First, check the releases tab to see if there is a binary available for your OS. If there is, just download that and put it in your path.

Otherwise, you will have to compile clip yourself with Cargo. The following commands should work to build clip:

```
$ git clone https://github.com/BookOwl/clip.git
$ cd clip
$ cargo build --release
```

The compiled binary can be found in `target/release/clip`. Just move it into your path.

## Usage
Using clip is really easy. To copy text to the clipboard, run clip with the `-c` flag and provide the text you want to copy to stdin. To paste text from the clipboard to stdout, just run clip with the `-p` flag.

You can find full usage info by running `clip --help`

## License
clip is released under the GPLv3. See LICENSE.txt for details.
