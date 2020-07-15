## Given the application has run when a file exists then it should check the last modified date so as not to overwrite the data.

1. Google for a `iso date now`
2. Using https://greenwichmeantime.com/info/clocks/iso/ get the current date.
3. Create a file in the root of the project with this date. Going to call this "last_modified.txt"
4. Attempt to use https://doc.rust-lang.org/std/fs/fn.read_to_string.html but this appears to be specific to address types, as it threw the [AddrParseError](https://doc.rust-lang.org/nightly/std/net/struct.AddrParseError.html)
5. [Opening a file to read](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html) but this also throws a can't find file error.
6. Having the project setup as a workspace seems to mean that the root of the project is actually the root of the workspace so moved under rust-web-template/
7. dbg the file returns the metadata - we want the contents!
8. Using the string parsing method in step 5 we can print out the date as a string.
9. This isn't really a date though since it's a string we have read from a file...
10. Checking the standard library and there doesn't seem to be any date handling functions aside from [Duration](https://doc.rust-lang.org/std/time/index.html)
11. Running `cargo add chrono` installs a crate which can handle creating datetimes for us.
13. We want to write the time once we've generated it so that is also possible using this [method](https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html)
14. Generating the current time now, writing to the file throws a type error because the write_all method is expecting out date to by in bytes or `&[u8]`
