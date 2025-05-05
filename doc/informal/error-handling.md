I still don't have a clear idea of how to do proper error handling in Rust so I'm going to write down some general rules.

## Functions/macros

Obviously any of these functions are okay in tests and const code, as if they fail there I will know they have failed and be able to fix the issues.

### `unwrap`

Don't use. A lot of current code uses `unwrap`, which has the potential to cause errors. This is no longer some loose Python code that can be changed and ran again, this needs to be reliable (at least the layer that gets game data) since it's being shipped as an executable.

### `expect`/`panic!`/`assert!`

`panic!` should only really be used where I know it's going to be hit in tests (or even just general day-to-day running) and none of the more informative macros fit the use case. E.g. any `CacheableVersionData` initialisations are fine since I know full-well they'll be hit in tests, but unless I know I have 100% coverage I shouldn't use them (e.g. I can also do these in `Stage` since that is exhaustively tested, but not in encounters since that isn't). `assert!` and variants follow pretty much the same rules as `panic!`.

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
