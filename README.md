Read the [user manual](doc/user-manual.md).

## Running the program

The user manual contains information about how to use the program, but it doesn't describe how to start it up. You will either need:
1. A [Rust compiler](https://www.rust-lang.org/tools/install).
2. An executable. I put one in the wiki mod chat a while ago and might do it in the future when this is more stable.

You will also need access to the game files. These are data mined and accessible to wiki mods, but if you're not one you'll need to find your own data mines. Your best public option is probably BCU, although BCU's folder structure is very different to the files that we have for wiki work. Unfortunately, due to PONOS crackdowns, we don't want to post the files on the wiki like we were able to do in the past.

## Why make this

### Why rewrite it in Rust?

1. To make use of Rust's robust security features and modern language design, so that I could both fix bugs I made from Python being way too loose, and make it much easier to add new stuff.
2. To make the program more efficient.
3. I wanted to rewrite it in Rust lol.

### How effective has this been?

1. Definitely worked. Technically I didn't need a whole new language, but Python makes it so easy to make a mess. I was able to properly separate out raw data, intermediate data that's much easier to work with, and wiki data. For example, in Python, my `Stage` object contained a whole load of stuff&mdash;there were 41 different fields just in the definition of the class, and iirc there were even more that I didn't have in the class definition. This mixed up stage identification, stage data, static data and wiki data.

   While Encounters probably didn't benefit much, Stage Info is significantly better-managed, and I've both fixed bugs and added new stuff. The treasure algorithm in Python was copied from BCU and I kept getting weird behaviours, mainly because BCU's code was so fragmented and difficult to follow. Rust's type system let me figure out how treasure really worked, and so I was easily able to fix many treasure-related bugs from the Python version. In addition, I also was able to add max clears to the infobox, because I was able to easily get the information rather than having to rewrite anything.

   As a final benefit of Rust, having built-in testing capabilities really helps. It's probably also a thing of my Python code being stored on the wiki rather than on github, but my Python code was only really testable by empirical measures. Rust made it so easy to just add in unit tests straight in the project, so when I broke something I found out immediately.
2. Stage Info it doesn't really make a difference, but Encounters it's slightly noticeable that the Rust version is faster. It's particularly obvious on Ms. Sign, who takes a few seconds in Python but probably under 1 second in Rust. It's also probably a bit due to architectural decisions making iteration in Rust much easier, but still it's faster.
3. Yeah worked. Got to learn a whole lot about Rust, and got to experience using Rust to write an actual program. Besides the strictness, testing and speed, I really like just the general feeling of writing Rust.

### What's better about Python?
1. Python is ***so much*** easier to write something quickly. Notably, any of the old `BCData` classes &ndash; the entire file is under 200 lines. [`stage_page_data`](src/wikitext/data_files/stage_page_data.rs) alone is bigger than that. Partially it's due to me trying to write stuff in a Rust-y way rather than sticking everything in a `dict` like I did in Python, but it was really draining nonetheless.

   Another major problem is Regexes being so much more complex: it probably took longer to write the `cleanup` function from Encounters in Rust than it did in Python, because in Python you can kinda think up a quick script and then just put it in code, rather than think about it like you need to do in Rust.

   I think my original intention was to do a bit more, like adding some Python scripts (StoryInfo, maybe some of the other runnable scripts, maybe update, putting in the pre and post fields for `TemplateParameter`, adding in literally any of the stuff in the user manual's example script) into Rust, but that would just be way too much effort. Stage Info and Encounters are major programs with moving parts that need to constantly be maintained, so they're worthy of Rust, but scripts that just involve reading through a few files and turning that into information are going to stay in Python.
