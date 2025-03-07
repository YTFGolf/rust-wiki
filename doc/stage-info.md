Stage info generates stage info.

## Input
Input can either be done with the cli or through stdin. Basically, you can run `rust-wiki stage l 0 0` and it will do the same thing as if you did `rust-wiki stage` and then inputted `l 0 0` after the first prompt.

## Format
Stage info uses "selectors" to work. For most stages, these will follow the format:
```bash
code mapnum stagenum
```

For example, Earthshaker would be `sol 0 0`. Available selectors for each stage type will be shown if you run `rust-wiki stage --sel` or `rust-wiki stage -s`.

Each line uses pipe characters to separate them. For example,
```
SoL|0|N|RN                      Stories of Legend
```
means that valid selectors for Stories of Legend are `SoL`, `0`, `N` and `RN` (i.e. `SoL 0 0`, `0 0 0`, `N 0 0` and `RN 0 0` will all give you the information for Earthshaker). Selectors are case-insensitive.

Selector values usually come in the form `CommonName|number|code_without_r|code_with_r`. Some types will have multiple common names, and some will have none.

## Other formats

There are two other methods for stage info than selectors: file and reference.

File simply uses the stage's information file name. For example, Earthshaker's stage information file name is `stageRN000_00.csv`. These are specifically tested to ensure that every single information file returns the exact stage that uses that file. This is mainly useful internally.

Reference uses the battlecats-db reference. This can be of the form:
- `*https://battlecats-db.com/stage/s00000-01.html`
- `https://battlecats-db.com/stage/s00000-01.html`
- `s00000-01`

## Obtaining numbers
Stage info requires the internal map and stage numbers to work. Unlike the selector, you can put as many leading 0s before the map and stage numbers. The two main ways you can find these are:

- Looking at the stage name image below the enemy base image. For example, on Earthshaker this image is `[[File:Mapsn000 00 n en.png]]`. Therefore, you can use `n 00 000` to get Earthshaker's stage info. This method is not foolproof, since there are many pages which reuse images from different stages, but it's generally a good place to start.
- Looking at the battlecats-db reference at the bottom of the page. Earthshaker's reference is `https://battlecats-db.com/stage/s00000-01.html`:
  - The first two digits are the stage type. You'll need to remove leading 0s, so this would give you `0`.
  - The third to fifth digits inclusive are the map number.
  - The last 2 digits are the stage number, but with a 1-based index. You'll need to subtract one, which would give you 0.

  Therefore, with the calculation complete, you can get the reference to be `0 0 0`. Of course, you could just put the reference directly into stage info and get the same result.

  HackerCatBOT actually uses these references, so most of them should be accurate as long as the page has exactly one reference complete with the stage number. However, not every stage might have a reference.

If both of these methods fail, or give weird results, the last resort is to Ctrl+f on [StageNames.csv](https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/data/StageNames.csv) (or your local copy) and look up the stage name.

## Other information

### Limited stage types

Certain stage types don't require a full path, such as when the stage types have only 1 map or stage. Stage types with 1 stage are:

- Filibuster Invasion
- Challenge Battle

These stage types don't require a map or a stage number, so `rust-wiki challenge` will act as if you have inputted `rust-wiki challenge 0 0` (which will also give you the same result you would have if you input `rust-wiki challenge 999 999`).

Stage types with 1 map are:

- Aku Realms
- Labyrinth

Similarly, these ignore the map number, so `rust-wiki aku 20` is equivalent to `rust-wiki aku 0 20`. Due to how the parser works, `rust-wiki aku 1 2 3 4 5` is equivalent to `rust-wiki aku 5`.

### Main chapters

For convenience, main chapters not only include normal selectors, but also individual subtype selectors:

- `main` and `3`: work like normal, i.e. `main 4 5` is the 6th stage of the 5th map (ItF Chapter 2 Norway).
- `eoc` is equivalent to `main 0`, and works the exact same way that The Aku Realms and Labyrinth work.
- `itf` and `w` refer to Into the Future. `itf 1 2` will be the third stage of the first chapter, and is equivalent to `main 3 2`.
- `cotc` and `space` refer to Cats of the Cosmos. `cotc 1 2` is equivalent to `main 6 2`.

A final thing worth mentioning is that `main 1` and `main 2` don't actually refer to any stages. If you want to find Empire of Cats's Moon stage, then `eoc 47` is chapter 1, `49` is chapter 2, and `50` is chapter 3.

A similar problem occurs with EoC Zombie Outbreaks, where the file name for Chapter 2 Moon is, for example, `stageZ01_49.csv`. However, since the rest of the game files seem to assume it's stage 48, this input will automatically be converted to the appropriate number.
