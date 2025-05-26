I still don't have a clear idea of how to do proper error handling in Rust so I'm going to write down some general rules.

## Functions/macros

Obviously any of these functions are okay in tests and const code, as if they fail there I will know they have failed and be able to fix the issues.

### `unwrap`

Don't use. A lot of current code uses `unwrap`, which has the potential to cause errors. This is no longer some loose Python code that can be changed and ran again, this needs to be reliable (at least the layer that gets game data) since it's being shipped as an executable.

### `expect`/`panic!`/`assert!`

`panic!` should only really be used where I know it's going to be hit in tests (or even just general day-to-day running) and none of the more informative macros fit the use case. ~~E.g. any `CacheableVersionData` initialisations are fine since I know full-well they'll be hit in tests, but unless I know I have 100% coverage I shouldn't use them (e.g. I can also do these in `Stage` since that is exhaustively tested, but not in encounters since that isn't).~~

Okay that's not true, since these tests rely on runtime data. Panicking in a `const` context is obviously fine since it won't compile otherwise. Tests in [data.rs](../../src/meta/stage/stage_types/data.rs) allow for establishing some invariants that allow `panic!`s in other places because it is physically impossible for compiled code to pass tests without being able to avoid the panics at runtime. But for game and wiki data panic should not be used, since although I can guarantee soundness for a particular version and particular files, they cannot guarantee it will be fine for every run of the program.

`assert!` and variants follow pretty much the same rules as `panic!`.

`expect` is a weird one, as far as I can tell it should be used in places where I'm confident the happy path is the only path (i.e. the same situations as `unreachable!`). The reason given in the expect should tell me why I would reasonably expect it to always succeed, such as `s.split(',').next().expect("split iterable should have at least one item")`.

### Acceptable

- `unreachable!` - path that should never be reached. For example
    ```rust
    if i < 0 {
        return None;
    }
    match i {
        0 => Some(2),
        1 => Some(3),
        2.. => Some(4),
        _ => unreachable!()
    }
    ```
    This is a bit contrived but we already checked the case where `i < 0` so the last case could never happen. If not obvious then `unreachable!` should have a reason, similar to `expect`.

- `todo!` - useful in dev and any place which has an issue filed for a missing feature.
- `unimplemented!` - used where a feature is not implemented and implementing the feature is not a priority.

## Patterns

The main pattern that should be used is bubbling. If I fail to read the file `a.csv`, I should bubble up an error that says there was an ioerror on `a.csv` along with the exact io error that was returned. The function calling can then choose to either handle it, or bubble up that this was in fact an error that stems from the reading function and pass the error given in the previous stage. Eventually, the code will probably end up panicking, but when it does I'll be able to see a full trace of all the errors and where it all went down to. Basically, rather than just a certain file had an error when reading, it'll show me the series of events that lead to trying to read that file so I can also figure out how to reproduce the error.

I should be very careful about how I implement error handling. In particular, if multiple parts could go wrong, I need to return a `Result` with proper reasons for each failure. Sometimes I might tend towards using an `Option` when that removes useful information.

The way I want to sum it up is that basically, just imagine that I'm trying to run the current code on a server 24/7: errors need to be propagated out of library code so that interface code can decide what to do with it. I can't just let the entire server go down because it couldn't find a file, so that error must be bubbled up with more and more context each time until the actual server receives that error, at which point it needs to be able to log the error with the full context.

That being said, I'm not trying to run this on a server 24/7, so if it takes a significant amount of work to change the structure it's probably not worth it. I mainly just need to think about the binary (redistributing it is a pain). Static wiki data panicking is a low priority, as if the panic is hit then the data is probably wrong. Game data will be a little more strict, but if all `CacheableVersionData` structs pass all tests using the current game version then probability of runtime panicking is low.

## 2025-05-26

Okay, more insight about how to display an error. Let's take this function:

```rust
pub fn from_file_name(
    selector: &str,
    version: &'a Version,
) -> Result<StageData<'a>, FromFileError> {
    match parse_stage_file(selector) {
        Ok(id) => Self::from_id(id, version).map_err(FromFileError::DataParseError),
        Err(e) => Err(FromFileError::InvalidSelector(e)),
    }
}
```

Now, `Self::from_id` will return an error, which is either that it couldn't open the correct file or it could open the file but couldn't parse it. In the first case, it must also report the name of the file it couldn't open, because any user of `Self::from_id` won't have that information. In the second case, it must report both the name of the file and the line where it occurred, because users of the function do not have that information.

`Self::from_file_name` will return a different error; either that same error from `Self::from_id` (but wrapped because it has to be), or an `InvalidSelector`. The trick here is that it only returns the error and not the name of the file. Why? Because the user **does** have that information. They called this function using that file name. In this case, the user of this call is responsible for adding necessary context.

In the first case, if the function fails, the user of the function needs to know why it failed. Perhaps there could be better error variants, but it does describe the aspects of the implementation that failed. The specific file that it failed on and the line it failed on are both implementation details. With `Self::from_file_name`, on the other hand, the file that couldn't be parsed into a selector is not an implementation detail, it is part of the interface.
