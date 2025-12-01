[![builds.sr.ht status](https://builds.sr.ht/~apoelstra/taskwarrior-hooks.svg)](https://builds.sr.ht/~apoelstra/taskwarrior-hooks?)

If you are seeing this on Github, **this is not the repository for this project**
and it is **not** maintained here.

Repository: https://git.sr.ht/~apoelstra/taskwarrior-hooks
Bug tracker: https://todo.sr.ht/~apoelstra/taskwarrior-hooks

# Andrew's TaskWarrior Hooks

**Installation:** Bulid the hooks with `cargo build --release`. Whatever hooks
you want, find them in `target/release/` and link them into `~/.task/hooks`,
changing the name as approriate.

For example,

```
ln -s "$PWD/target/release/on-add-relative-uda" "$HOME/.task/hooks/on-add.relative-uda"
```

Or copy them. Whatever. Importantly **you have to change the name of the file.** I
cannot produce binaries with `.`s in them because Cargo does not support this.

## on-add.relative-uda

Lets you set the `wait` and `until` fields of recurring tasks relative to your `due`
date. `until` is taken to be a duration **after** the due date and `wait` is taken
to be a date **until** the due date.

**Installation.** Add two UDAs, `relwait` and `reluntil`, to your `~/.taskrc`.

```
uda.untilrel.type=duration
uda.untilrel.label=Until (relative)
uda.waitrel.type=duration
uda.waitrel.label=Until (relative)
```

Then install this hook, and for any recurring tasks you can set a relative wait time.
For example,

```
task add project:Home.HVAC due:2026-01-15 recur:quarterly relwait:4w 'Replace furnace filter'
```

sets a "Replace furnace filter" task to occur quarterly on the 15th of the first month of
the quarter, with the task hidden until 4 weeks ahead of time.
