# Notes
Personal journal in terminal.

**This is still work-in-progress. Please check back in a while!**

This project is inspired by [jrnl](https://github.com/maebert/jrnl), although it aims at more interactive behavior.

Also, it's a way to improve my Rust skills.

# Incoming features

* Journal entries or quick notes in terminal (obviously).
* Optional encryption.
* Some nice way of searching the entries.
* Interactive management console supporting history and tab-completions.

# More random ideas

* Data probably kept in text file.
* More interfaces, perhaps NCurses or web.
* Notes stored in "books" that can be one of:
    * Journal - dated entries
    * Notepad - wiki-like structured notes
    * Scratchpad - just one big entry
    * ...more? (To-Dos, some-sort-of-brain-dump-thing, ideas, ...?)
* Basic navigation similar to Linux shell.
* Statistics and streaks to encourage daily updates.
* The first line is the title of the entry (for most cases at least).

## Interface ideas

* Basic management commands for books and entries:
    * ls, cd, cat/view/show(?), add, edit, rm, grep/find(?)


Books listing:

```
> ls
my_journal
random_thoughts
scratchpad
```

Entries listing:

```
> my_journal
my_journal> ls
Last 5 entries:
42    Example title       24-01-2016 23:44:41
41    Lorem Ipsum         22-01-2016 11:23:12
40    Dolor Sit           10-01-2016 12:32:00
39    Happy New Year      01-01-2016 00:02:02
38    Some random entry   01-01-2016 00:01:00
my_journal> 42
24-01-2016 23:44:41
Example title

This is an example entry.
```
