# About

The main objective is to reformat old text files that I collected from BBS days (e.g. Hitel) so that I can use ebook-convert from Calibre to make epub out of.

This program does two things. 

1. Find a paragraph by combining multiple lines
1. Split each paragrah into sentences if possible


## 1. Find a paragraph by combining multiple lines

Main idea is to combine multiple lines into one line if they look like one paragraph.

The stage collects multiple lines until it finds *end of the sentence* and combines the lines so far into one string. 
A line is considered as the end of the sentence, if it ends with any of the following: `., ", ], ), -`.

## 2. Split each paragrah into sentences if possible

In this stage, 'sentence' is defined as a block of text ending with `.` followed by space.
Between a pair of sentences, `\n` is added.
This stage is needed because I found that `ebook-convert.exe` from `Calibre` have problems with really long lines.


# How to use

Convert a text file (which should be in UTF-8).

```
combine_lines input.txt output.txt
```


Then use `ebook-convert.exe` from `Calibre` to make epub.

```
ebook-convert output.txt output.epub --paragraph-type="single" --insert-metadata --no-default-epub-cover --flow-size=200 --title="Book title" --authors="Author Name"
```
