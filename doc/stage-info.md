Stage info generates stage info.

## Input
Input can either be done with the cli or through stdin. Basically, you can run `rust-wiki stage l 0 0` and it will do the same thing as if you did `rust-wiki stage` and then inputted `l 0 0` after the first prompt.

## Format
Stage info uses "selectors" to work. For most stages, these will follow the format:
```bash
code mapnum stagenum
```

For example, Earthshaker would be `sol 0 0`. Available selectors can be found in [stage_metadata](../src/data/stage/raw/stage_metadata.rs#165) just below where it says `static STAGE_TYPE_MAP`, and are used in the `initialise_type_map` function. Each line uses pipe characters to separate them. For example,
```rust
        initialise_type_map("SoL|0|N|RN",                               T::SoL),
```
means that valid selectors for Stories of Legend are `SoL`, `0`, `N` and `RN` (i.e. `SoL 0 0`, `0 0 0`, `N 0 0` and `RN 0 0` will all give you the information for Earthshaker). Selectors are case-insensitive.
<!-- Note: need to update line number whenever stuff changes -->

Possible selector values come in the form `CommonName|number|code_without_r|code_with_r`, with the exception of main chapters. Some types will have multiple common names, and some will have none.

## Other formats

There are two other methods for stage info than selectors: file and reference.

File simply uses the stage's information file name. For example, Earthshaker's stage information file name is `stageRN000_00.csv`. These are specifically tested to ensure that every single information file returns the exact stage that uses that file. This is mainly useful internally.

Reference uses the battlecats-db reference. This can be of the form:
- `*https://battlecats-db.com/stage/s00000-01.html`
- `https://battlecats-db.com/stage/s00000-01.html`
- `s00000-01`

## Numbers
Stage info requires the internal map and stage numbers to work. Unlike the selector, you can put as many leading 0s before the map and stage numbers. The two main ways you can find these are:

- Looking at the stage name image below the enemy base image. For example, on Earthshaker this image is `[[File:Mapsn000 00 n en.png]]`. Therefore, you can use `n 00 000` to get Earthshaker's stage info. This method is not foolproof, since there are many pages which reuse images from different stages, but it's generally a good place to start.
- Looking at the battlecats-db reference at the bottom of the page. Earthshaker's reference is `https://battlecats-db.com/stage/s00000-01.html`:
  - The first two digits are the stage type. You'll need to remove leading 0s, so this would give you `0`.
  - The third to fifth digits inclusive are the map number.
  - The last 2 digits are the stage number, but with a 1-based index. You'll need to subtract one, which would give you 0.

  Therefore, with the calculation complete, you can get the reference to be `0 0 0`. Of course, you could just put the reference directly into stage info and get the same result.

  HackerCatBOT actually uses these references, so most of them should be accurate as long as the page has exactly one reference complete with the stage number. However, not every stage might have a reference.

If both of these methods fail, or give weird results, the last resort is to Ctrl+f on [StageNames.csv](https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/data/StageNames.csv) (or your local copy) and look up the stage name.

## Main chapters
> Note: There is a planned reform of stage info that would change how selectors work under the hood. This shouldn't affect basic usage of this program, although it could improve some selectors.

Main chapter selectors are significantly different from normal stages.

- EoC: `eoc {num}` - gives EoC stage `num + 1`. Moon works differently: `47` gives chapter 1, `49` gives chapter 2, and `50` gives chapter 3.
- ItF: `itf {chap} {num}` - chap is 1-based, i.e. `itf 1` means chapter 1. num is still 0-based, so `47` means Moon. `W` also works as an alias for `itf`.
- CotC - same thing as ItF basically. `Space` is an alias for `cotc`.
- Outbreaks: `Z {chap} {num}`. chap is 1-based (`1` is eoc 1, `4` is itf 1 etc.), num is 0-based.
- Filibuster - `filibuster` is the invasion. There is no need for any numbers.
- Aku Realms: `aku {stage}` - e.g. `aku 0` is Korea. `dm` is an alias for `aku`.
