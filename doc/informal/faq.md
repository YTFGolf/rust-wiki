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

stagemeta
should have been split apart individual bits, e.g. if only needs variant then should only use variant rather than full thing.

## Is Rust better than Python?

I'd call Rust _more scalable_. If you want to do something simple, Python is absolutely the better choice. I still write Stage Info scripts in Python, such as when I want to make a stage page by clicking a button. The only real difference is that instead of doing it by `import StageInfo` I now do it by `import subprocess` and use regexes to make any alterations.

However, Rust's design has just forced me to make better code for a project that needs to scale well. Stuff like sum-type enums making me consider every variant, constructors not existing, and built-in testing have made it just easier to write maintainable, future-proof code (those adjectives together make me sound like a tech bro trying to sell you something and it's disgusting, but it's also 100% true. Ugh).

## Appendices

Yes I'm fancy like that and have appendices.

### Appendix A

This appendix is about using the typing system for meaning.

Treasures in TBC have a complicated system. They are contained inside the MapStageData file alongside timed rewards.
