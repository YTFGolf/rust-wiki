//! Deals with cat/enemy abilities.

use crate::generate_ability_enum;

struct Config {
    is_general: bool,
    is_cursable: bool,
}

generate_ability_enum!(
    #[doc = "Cat or enemy ability."]
    name = Ability,

    #[doc = "Freeze the enemy."]
    {
        name = Freeze {
            #[doc = "Chance to freeze the enemy."]
            chance: u8,
            #[doc = "Duration of freeze in frames."]
            duration: u32,
        },
        config = Config {
            is_general: false,
            is_cursable: true,
        }
    },

    #[doc = "Double money collected when defeating the enemy."]
    { name = DoubleBounty, config = Config { is_general: true, is_cursable: false } }
);

// Check Donut's `multab`.
// I'm not sure this is a good idea tbh.

impl Ability {
    /// Does the ability have targets when used on a cat. This is equivalent to
    /// [`Ability::is_cursable`].
    pub const fn has_targets(&self) -> bool {
        self.is_cursable()
    }
}

/*
This is done as a macro so that properties as in [`Config`] can be intrinsically
tied to the enum parameters.

If this gets too unreadable it's also entirely possible to either A) inline it
completely then format that B) create a procedural macro that allows the syntax
to be completely custom.

Migrating it to another format wouldn't be quick (well I could write a Python
script to do it) but it would probably be quite easy, so it's not really much of
a concern that this is getting too esoteric.
*/

/*
# Comparison of different formats

## Enum with pattern matching

E.g. what this macro compiles to.

Pros:
- It's just Rust code.

Cons:
- It means that the data is no longer intrinsically linked to the enums: I have
  to update the functions and the enum separately.

Ngl I'm not sure that the con is really that bad.

## Declarative macro (current)

Pros:
- One of the easier options to set up.
- As long as I'm not doing too much the definition isn't too bad.

Cons:
- Errors show up in [`super::ability_defined`] rather than in this file.
- Reading definitions from this file is really hard.
- Formatter doesn't work.
- Declarative syntax is quite strict.
- Have to use `#[doc]` rather than being able to use `///`

## Procedural macro

Easiest syntax, e.g.
```rust
#[derive(Debug, AbilityConfig)]
pub enum Ability {
    #[ability(is_general = false, is_cursable = true)]
    Freeze { chance: u8, duration: u32 }
}
```

Pros:
- Much more powerful and flexible than declarative macros.

Cons:
- Difficult to set up; might not be worth it for simple use cases.

## Trait

Pros:
- Idk can't remember

Cons:
- Afaik would have to define every possible method over here, including stuff
  like "ability name as string".
- Trait objects would have to still keep track of concrete type so it's
  basically just a less ergonomic enum.

## Enum with fake types

Something like `{ DoubleBounty { is_general: True, ... } }`.

I think this would create a lot of clutter, e.g. instead of having
`abilities.push(DoubleBounty)` you'd need to initialise it.

## Similar setup to stage types

I.e. where I have the enum definition, then I have the variants in an array
showing config options, and I use tests to enforce invariants.

This wouldn't work as well because these variants might also need to store data,
which would be difficult to represent in an array, which would basically force
me to use another macro.

## Conclusion

Overall, I like the ability to be able to immediately see `Freeze`'s config
right next to its definition, so anything I do must involve having the config
next to the enum. Right now I have the derive macro and I'm fine with that.

Possible next steps are:
- `match` that does like `Self::Freeze { .. } => { Config { is_general:false,
  is_cursable:true } }`, ngl should have done that originally.
- proc macro, would be much easier to use but also needs to be created which
  requires building a real parser.
*/
