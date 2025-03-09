These aren't really frequently asked questions, I just thought that was probably the name that covered the subject here.

## Why rewrite in Rust?

I had this in a commit before but I removed it because it was rambling. But this part of the docs is 100% rambling! Might as well rewrite the why just like I rewrote the project.

Basically, I felt like it.

I'd written a few Rust programs before, but just toy projects. The most substantial thing I did with it was writing some Advent of Code solutions in Rust. I liked the syntax and the typing system, and it seemed like it was just a very interesting language that looked at all the past languages and tried to figure out what was done right and what was done wrong.

The old version of my stage code was in Python, and it was a complete mess. I mainly just did things that seemed to make sense at the time but kept on causing problems down the line. For example, in `StageInfo` I wrote something that would allow `self.x` to translate to `self.stage.x`, which seemed convenient at the time. However, later on, I started to think it was too much wishy-washy magic, but now my code was completely dependent on doing that and literally broke when I tried to remove it. There's also the fact that I literally stored the entire thing in a tabber on the wiki rather than in a git repository like a normal person.

So combine my interest in Rust and a messy codebase in need of rewriting, and you've got "Rewrite it in Rust™". This messy codebase was a rewrite of an even messier codebase, and I thought Rust's strict typing system would help me to make a codebase that won't be needing another complete rewrite.

Of course, the first thing I wrote just had to be a monolith that made basically the same mistakes as I did with the Python 🤦
