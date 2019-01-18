# About

Main idea is to combine multiple lines into one line if they look like one sentence.

Collect each line into a buffer and then combine all those in the buffer into one line if the current line seems like the end of a line.

Whether a certain line is the end of the sentence or not is based on simple rules.
A line is considered as the end of the sentence, if it ends with any of the following: ., ", ], ), -.

This means that each line is not guaranteed to be one sentence.
It could contain more than one sentence but at least it will end with one of the characters above.

# How to use

```
combine_lines input.txt output.txt
```

# Use case
The main use case is to reformat old text files from BBS days (e.g. Hitel) so that I can use ebook-convert from Calibre to make epub out of.

```
ebook-convert input.txt input.epub --paragraph-type="single" --insert-metadata --no-default-epub-cover --flow-size=200 --title="Book title" --authors="Author Name"
```

However, I found that ebook-convert does not work with really big text files for some reason, so this is not as useful as I hoped it will be.

My ad-hoc test with a Korean text file in UTF-8 shows that the `paragraph-type` argument works depending on the file size

* 253848: works
* 253955: does not work

Also the result is different if the text file is in English (UTF-8).
