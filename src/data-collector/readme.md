# Requirements

This is not as high level as it might be, nonetheless you can [read more about this here...](https://martinfowler.com/bliki/GivenWhenThen.html). Normally this is done to construct behaviour style tests.

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
14. Generating the current time now, writing to the file throws a type error because the write_all method is expecting out date to be in bytes or `&[u8]`
15. Converted to bytes using `as` method, successfully saves the file.
16. TODO...

## Given the secret is set and the api id is set when the application is run then endpoint should be called and the data returned.

1. `cargo add dotenv`
2. `use dotenv;`
3. Set the key and use `std::env::var` to obtain the environment variables that have been set by calling `dotenv`.
4. create seperate functions for each operation and return the val
5. watching out for the match arm types having to be the same, used panic macro to get around this.
6. Added femme logging as it might be useful later.
7. started writing fetching data function, also added return types since the get functions now return something, missed a `<` in the return type but things compiled once that was added in.

## Given the has been called when the data is returned then it is written to a file
1. Tried creating a writer and passing it to serde: 

``` Rust
fn write_json<W: io::Write, T: serde::Serialize>(data: &str) -> serde_json::Result<()> {
    let writer = &File::create("/tmp/phemex-response.json");
    serde_json::ser::to_writer(&mut writer, &data)?;
    Ok(())
}
```
but this has thrown errors that:

``` Bash
error[E0277]: the trait bound `&std::result::Result<std::fs::File, std::io::Error>: std::io::Write` is not satisfied
    --> src/data-collector/src/main.rs:52:5
     |
52   |     serde_json::ser::to_writer(&mut writer, &data)?;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::io::Write` is not implemented for `&std::result::Result<std::fs::File, std::io::Error>`
     |
    ::: /home/davidmaceachern/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_json-1.0.57/src/ser.rs:2154:8
     |
2154 |     W: io::Write,
     |        --------- required by this bound in `serde_json::ser::to_writer`
     |
     = note: required because of the requirements on the impl of `std::io::Write` for `&mut &std::result::Result<std::fs::File, std::io::Error>`
```
2. So a [search](https://www.reddit.com/r/rust/comments/9e793y/question_best_way_to_append_data_to_a_json_file/) has revealed another way to try is dumping as a string and writing this to a file since I already know how to write a string to a file. This might not be the prettiest way to do so however.

It appears the file doesn't exist so, checked file positioning, checked whether file can be created at all, why can't it create it? Try converting now() to iso format and string name?

``` Bash
running 2 tests
test tests::test_check_last_modified ... ok
test tests::test_write_json ... FAILED

failures:

---- tests::test_write_json stdout ----
thread 'tests::test_write_json' panicked at 'couldn't create ../tmp/2020-08-06 21:40:01.541467293 UTC.json: No such file or directory (os error 2)', src/data-collector/src/main.rs:56:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_write_json

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--bin data-collector'
[Finished running. Exit status: 101]
```


