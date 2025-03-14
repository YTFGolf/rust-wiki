These aren't really frequently asked questions, I just thought that was probably the name that covered the subject here.

## Why rewrite in Rust?

I had this in a commit before but I removed it because it was rambling. But this part of the docs is 100% rambling! Might as well rewrite the why just like I rewrote the project.

Basically, I felt like it.

I'd written a few Rust programs before, but just toy projects. The most substantial thing I did with it was writing some Advent of Code solutions in Rust. I liked the syntax and the typing system, and it seemed like it was just a very interesting language that looked at all the past languages and tried to figure out what was done right and what was done wrong.

The old version of my stage code was in Python, and it was a complete mess. I mainly just did things that seemed to make sense at the time but kept on causing problems down the line. For example, in `StageInfo` I wrote something that would allow `self.x` to translate to `self.stage.x`, which seemed convenient at the time. However, later on, I started to think it was too much wishy-washy magic, but now my code was completely dependent on doing that and literally broke when I tried to remove it. There's also the fact that I literally stored the entire thing in a tabber on the wiki rather than in a git repository like a normal person.

So combine my interest in Rust and a messy codebase in need of rewriting, and you've got "Rewrite it in Rust™". This messy codebase was a rewrite of an even messier codebase, and I thought Rust's strict typing system would help me to make a codebase that won't be needing another complete rewrite.

Of course, the first thing I wrote just had to be a monolith that made basically the same mistakes as I did with the Python 🤦

## Did it work?

Yes.

Now of course, as I said, the first thing I wrote was a monolith making the same mistakes as Python. It was also initially done when we were migrating to Miraheze, so it took a while for it to really become useful. Once it did become useful, I started seeing the benefits.

For one, there was now a clear flow of data. Instead of the `Stage` object doing everything from implementation details of the game all the way up to storing wiki-specific information, this was turned into multiple different structs:
- A `StageMeta` struct that contained metadata about a stage, including the type, map and file numbers, and the stage's file names. This struct itself was also a bit too big, as you'll see down below, but it's nothing compared to the monstrosity lurking inside the Python code.
- A raw `StageData` struct that stores raw data directly pulled from the files. Rust's `serde` crate is so much better than storing everything in arrays like I did when parsing stuff with Python. `serde-derive` essentially let me write a definition of the raw data and `serde` would deal with parsing the data, whereas with Python I specifically had to be like `self.energy = int(line[1])`.
- A `Stage` struct that provides high-level abstractions. While `StageData` showed raw data and was bound to a containing `Version` object, `Stage` had no references to `Version` and owned all of its data. I also managed to use the typing system to assign meanings: see [Appendix A](#appendix-a) because it's really long.
- A `StageWikiData` struct that is part of a module that provides functions to allow you to find the names of stages, maps and stage types (using `StageNames.csv`).

Was this a lot more complicated to set up? Absolutely. Was it worth it? Also absolutely. I fixed bugs, such as the Treasure one in the appendix; I was able to add more stuff such as max clears; and I found it much easier to add new stuff on top (such as maps) rather than basically having to write them from scratch like I would have with Python.

I will also say that this codebase is only a few months old, compared to the Python version which is years old and probably still needs light maintenance. Also not a whole lot of this section is unique to Rust and is more related to general principles of good design.

## Were there any difficulties?

Yes I'm aware this doesn't seem like a question a real human would ask but I made this part of the docs to ramble and ramble I shall do. I don't even know if _**I**_ will be reading this in the future.

The main difficult thing I will talk about is the `StageMeta` struct, which was incidentally the very first thing I wrote for the project, basically making the entire thing based off of this one struct that I ended up removing completely.

### In the beginning

This was getting really long so I split it into sections.

When I was beginning the project, I wanted to have something small that I could use as a single part of the Stage struct, that would just describe metadata such as the stage's numbers and the files its data was contained in. I defined some initial data like I did with the old Python code's `StageType` file, basically a map over every stage type describing the name, "code", number, and the "R" column, as well as another map which contained regex matchers.

The old `StageType` file contained a lot of stuff. I think initially it was just about parsing selectors, but then when I had Encounters I then added something to parse file names, then later on I added a db reference parser since it was actually pretty easy to do with a simple regex and quite useful. This probably sounds bad but like what else could I have really done. It's either a nightmare to implement or a nightmare to use. This way pretty much meant I only needed to change two lines if a new stage type was added, and my fragile spaghetti code would make sure it _just worked_.

### Rustifying

The original `StageMeta` took me like 2.5 days to implement and had about 1300 lines of code. The vast majority of this was testing. I tested if selectors were case insensitive, I tested invalid values, I tested every parse mode, I tested properties on randomised data. This might sound tedious but it basically meant the entire thing was airtight. If I accidentally broke the slightest thing, I would know immediately. Having testing built-in is great.

I also implemented an enum for the stage type, because why not use Rust capabilities. Instead of being like `if meta.code == "RN"` I could be like `if meta.type_enum == StageTypeEnum::SoL`, which is leagues better.

There was also an update that added support for having difficulties in main chapters, which eventually forced me to completely redefine the numbers. I'd been going off of BCU's numbers (note: NEVER assume BCU devs know what they're doing), which essentially said every sort of main story level had the type number 3. This is not the case in the game: only EoC, ItF and CotC proper have the type number 3. Outbreaks are 20, 21 and 22, Filibuster is 23, and Aku Realms is 30. I had to completely change that and add some more enums.

### Problems

The `StageMeta` at this point was pretty much a 1-to-1 translation of the functionalities of `StageType`. My equivalent of `stage_codes` was easier to use (i.e. actually used a function call rather than being a line in a string) and I had tests and enums, but overall it was just the same thing with some extra bells and whistles.

This was terrible. For starters, I couldn't just create a new `StageMeta` object. I had to initialise it from a selector. Using the raw numbers didn't work for main chapters. Even after my refactoring, all of my tests (more than 100) relied on these selectors when they didn't need to. There had to be substantial plumbing and error handling to deal with the Aku Realms invasion in the encounters module.

The second major problem occurred when I tried to start implementing a module to show map data, and I realised that I just didn't have anything capable of showing metadata about maps. So I just cheated and added `" 0"` to the end of the selector and reused `StageMeta`. However, this was too much information: e.g. if I had a function that needed data about map n would I make that take in the type and map numbers (inconvenient) or a `StageMeta` (means that if you don't already have the object then you need to create it even when you don't need its functionalities).

A minor problem was also just the sort of 'mental overhead' of creating a new `StageMeta` object. Every time I wanted to do it, I would have to add a new string into memory since it also initialised the stage file name and the map file name.

### Fixing it

I found the overreliance on strings annoying, but the only real pain point was the Aku Realms invasion; using selectors in tests was something I only realised was completely stupid once I'd started to replace old code. But when it came to maps I immediately realised I had to stop development and fix the mess that was `StageMeta`.

I got a bit annoyed at this section because it felt more like a boring recap of what's in the new thing, so consider this a warning that what follows is really boring.

#### IDs

The first thing I did was create "ID" structs. There were 3 of these: stage types (I called them "variants" because "variant" isn't a reserved keyword), stage maps and stages themselves. Each of these ID structs basically only had one job: identify something you're supposed to identify.

Variants (`StageVariantID`) were implemented as an enum with discriminants (basically, each variant corresponds to one type number, and you can easily switch back and forth between the `u32` and `StageVariantID` representations). This was mostly the same as `StageTypeEnum` except it split `Outbreaks` into `EocOutbreak` etc. because each of them has a separate type number in the game. Being mostly the same also made it really easy to convert old code to `StageVariantID` because all of the non-outbreak variants had the exact same name.

Variants also came with a few functions alongside converting the variant to a number: these were simple convenience functions that only needed to be defined in one place. As of time writing, all of these are simple "is questions" such as `is_main`, which checks if it's main chapters, outbreaks, filibuster invasion or aku realms.

Maps (`MapID`) are simply a struct with a `StageVariantID` and a `map_num`. It has a few more ways of initialisation, allowing you to create it from its parts, from raw numbers (the variant number gets converted before initialising), and from the `mapid` that you see in places like `Map_option.csv`. It contains fewer pure functions if you ignore the getters: it has one to get the `mapid` described earlier and it has one to determine the subtype in main chapters levels (eoc, itf or cotc).

Stages (`StageID`) work in a consistent way to `MapID`, containing a `MapID` and a `stage_num`. There's not much special about it, and that's how it should be.

#### "Stage Types"

For all their faults, `StageMeta` and the original `StageType` had one key benefit: adding a new type required 3 lines of code: one for the variant, one for the list of stage types, and one for the regex mapping (two lines for `StageType`). To be honest, I think a monolith is a necessary evil there.

I decided to go with a similar static variable containing everything. I slightly changed how the information was spread out.

The whole "map code" and "has r prefix" didn't really work. First off, I had to do something completely different for main chapters anyway, and for extra stages the map code was `"RE|EX"` which kinda goes against the whole "write explicit Rust" thing I've kinda been going for in this repo. Instead, I split this into a map code and a stage code.

Map code is an `Option<&'static str>` and is primarily the code used in `MapStageData` files. For all of the maps that don't have a `MapStageData` file (main, outbreaks, filibuster), this will be `None`. Stage code is a custom enum that allows me to define the different cases. `RPrefix` stages have stage data file names that begin with R. `Map` stages have stage data file names that use their map's prefix, which seems to be happening a lot with the more recent types. `Custom` stages have completely unpredictable stage data file names and must have custom logic. `Other` stages use their stage code rather than their map code for most purposes, and it's currently only EX stages that are `Other` (`MapStageData` files use `RE` but everything else uses `EX`). To assert invariants like a `None` mapcode happening iff stagecode is `Custom` I just used tests.

The other major thing is that I put the selectors inside the stage types static directly. Being inside the static meant the entire thing needed to be `LazyLock`ed but it also reduces the amount of lines of code needed. With separating out the outbreaks I was now able to calculate most of the selector parts at runtime, so all that the defined selectors needed to do was write the common name: the number, map code and stage code were added at runtime. I was also finally able to create a CLI option to show available selectors. I might have done other things but I hate this section and want to stop writing it.

#### Parse and transform

One of the major features of `StageMeta` was that you could use strings to initialise it, which was important for both the command line interface and for the encounters module which needed to loop through each individual stage. A second thing was that you could get the map and stage data file names, as well as the images used for the display names.

In order to try and avoid the mistakes of `StageMeta`, I created separate modules for parsing and transforming. `parse` would deal with stuff like selectors, db refs, file names, then turning them into variant, map or stage data. `transform` would do the opposite: it would take the variant, map or stage data and transform them into the formats they came from.

I don't really have much else to say about it, I implemented them and copied over basically every test from `StageMeta` into here, and made sure that `parse(transform(data))` was equivalent to `data` and the same the other way around.

### Deprecating the old version

Fully replacing `StageMeta` with ID objects actually only took a single weekend, once I'd properly written and tested everything. I tried to take a sort of "bottom-up" approach, where I replaced `StageMeta` with IDs in parts that didn't call other functions requiring `StageMeta` objects. In practice it ended up being a bit of a mess but I cleaned it up eventually.

The way the deprecation ended up going was I implemented `From<&StageID> for StageMeta` and basically just put `let stage_id: StageID = (&meta).into();` at the tops of functions that used `StageMeta`, and changed any functions requiring numbers to use the IDs instead. Once a `meta` field of a struct was only used to convert it to an ID I would then just remove `meta` from that struct and remove all of my old conversion stuff. Honestly it was quite boring and same-y. Then I deleted `StageMeta` completely when I was done with that.

<!-- need to update with deprecating the old selectors in tests -->

This whole question ended up way longer than I expected it to be so I'll probably end up putting this in its own file.

## Is Rust better than Python?

I'd call Rust _more scalable_. If you want to do something simple, Python is absolutely the better choice. I still write Stage Info scripts in Python, such as when I want to make a stage page by clicking a button. The only real difference is that instead of doing it by `import StageInfo` I now do it by `import subprocess` and use regexes to make any alterations.

However, Rust's design has just forced me to make better code for a project that needs to scale well. Stuff like sum-type enums making me consider every variant, constructors not existing, and built-in testing have made it just easier to write maintainable, future-proof code (those adjectives together make me sound like a tech bro trying to sell you something and it's disgusting, but it's also 100% true. Ugh).

## Appendices

Yes I'm fancy like that and have appendices.

### Appendix A

This appendix is about using the typing system for meaning.

Treasures in TBC have a complicated system that are probably boring so skip this paragraph if you don't care. They are contained inside the MapStageData file alongside timed rewards. If the line is longer than 15 columns and everything from `line[8]` to `line[14]` is `"-2"`, then it uses timed score rewards, which can also have a treasure drop. If it's not a timed score reward stage then the stage can have multiple treasures drop. If there are multiple treasures dropped then there will be a number ranging from -4 to 1 that dictates how that drop works.

Initially, in the Python, I just copied BCU's code. This was a mistake. BCU's code goes into the full Java thing of "make all the logic as difficult to follow as possible and don't document anything for job security".

#### BCU sucks, part 1

First off, here are the relevant pieces of code:
- [DefStageInfo](https://github.com/battlecatsultimate/BCU_java_util_common/blob/1df366ad04a77a44405c40d21c09b7999c61f8f9/util/stage/info/DefStageInfo.java#L36)
- [analyzeRewardChance](https://github.com/battlecatsultimate/BCU_java_util_common/blob/1df366ad04a77a44405c40d21c09b7999c61f8f9/util/stage/info/DefStageInfo.java#L155)
- [readDropData](https://github.com/battlecatsultimate/BCU-java-PC/blob/f8e32702cd5cb493e33ebbb34aacb9c0778cea8f/src/main/java/utilpc/Interpret.java#L1137)

`DefStageInfo`'s initial bit looks quite like my old Python: it just reads from the appropriate `MapStageData` line and stores all of the appropriate numbers into appropriate variables. I'm not a fan of Java making it difficult to tell what's local and what's a class variable but that doesn't begin to describe my hatred of this code.

Line 48 then reads as `once = data[data.length - 1];`. Now I checked the entire repo: THIS DOES NOTHING. THIS NEVER GETS USED EVEN ONCE. There's zero documentation of what it's used for if it's used for something outside BCU.

Then it does some fairly easy-enough-to-understand stuff, it just checks relevant entries if they all are `-2` as I stated earlier. Then we have `time = new int[(data.length - 17) / 3][3];`.

I hate this for the following reasons:
- You're assigning it to a variable that hasn't been initialised. This is why Java gets so many null pointer exceptions. Maybe if you hadn't stupidly made it `public final int[][] time;` then you would be able to initialise it properly.
- Why is this an array. Literally look at [this comment](https://github.com/battlecatsultimate/BCU_java_util_common/pull/36#discussion_r1970074891), Mandarin themselves says you shouldn't do this. This should be an `ArrayList` and you should be pushing to it.
- Why is this an array of arrays. Each item in `time` is an array of `[score, item_id, item_amt]`. WHY COULD YOU NOT JUST MAKE EACH ITEM A NORMAL OBJECT.

The next loop doesn't contain much I haven't already complained about. For some reason it does `for (int j = 0; j < 3; j++)` rather than just making an object, but that was the last point in the previous thing. Then there's at least an else that initialises `time` which ig prevents a null pointer exception but why not just make it an empty arraylist.

Then we have drop rewards (treasures). It begins with some fairly innocuous stuff, then intitialises an empty array which I've already talked about. Then there's `rand = 0`. Zero explanation of what `rand` is and it'll definitely come up later so just you wait. The next branch then does a similar thing and creates an array of length... 1? The next branch does nothing wrong that I haven't already complained about.

Then it does
```java
if (drop.length > 0)
	drop[0] = new int[] { data[5], data[6], data[7] };
```

... what? why? why could you not just have done this when initialising this? Are you really that concerned with repeating yourself you'd rather make the code completely unreadable?

#### BCU sucks, part 2

Yep this is also becoming an essay.

So, `analyzeRewardChance`. Starts off fairly alright... hold on is that an ArrayList I see? Where was that in the previous function? I don't actually have that much to say about `analyzeRewardChance`. The only real thing is that imo it should be initialising `res` inside each if block and returning it at the end, but maybe java sucks and can't be relied on.

What I will complain about is how this is utterly terribly designed, and for that we need to look at `readDropData`. It begins with some fairly simple stuff, getting lists and adding null guards because java sucks. Then we have the loop and my god I hate the loop.

Understanding the loop requires an understanding of `analyzeRewardChance`, which... why? Why does a function in a git submodule with zero documentation dictate how this loop works? It uses the special value of `[]` to mean that all the items have an equal chance of dropping. But this is not communicated in any way.

Except here's the thing: it also uses this when `rand = -3`. Is this intended behaviour or is this a bug? I have no clue, but based on the fact that this was probably written before Infernal Tower and that Infernal Tower stages have drop reward chances like `[33, 33, 34]`, and also the fact that there are way too many dumb assumptions in the code, I'm gonna assume it's a bug. Although tbf, it is what the [official data](https://www.youtube.com/watch?v=oWUZjIbgcao) says.

In fact, I'm convinced that BCU doesn't understand `rand = -3` because it also doesn't know that these rewards only appear once. It also doesn't appear to know that you can't use Treasure Radars for `rand = -3`.

if it's empty then it gives you the index of the reward
treasure radar bug
exiting on empty
coding should be simple. If you rely on some invariants, make it clear what those are
Rust is the only thing that let me make sense of BCU's terrible code

<!-- I cannot believe this has gotten to like 3k words in 3 days, when only working on it late at night. If I could have had this productivity when doing my third year project... -->
