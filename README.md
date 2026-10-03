# sbc

`sbc` is a simle command line tool that counts the statements and blocks of source files whose programming language is from the C family.

What does 'sbc' stand for?
It means "statement-block counter".
I couldn't come up with a better name (I'm not that good at naming things).

## Why this exists

Some time ago, I saw stream by Tsoding called [Classy Little Scripting Language](https://youtu.be/G4Ugv18tJ5E&t=180) which showcased the [Wren](https://wren.io/) programming language.
In it, there was a brief discussion about how the authors of Wren measure the complexity of its VM implementation using semicolons instead of lines of code.
This then gave me the idea of making a program that would count the number of statements in a program via semicolons, but also the number of blocks via an ending curly brace as well, to estimate the complexity of a program.
I say *estimate* because obviously you cannot entirely determine the complexity of a program by number of statements alone, but it can give you a decent idea.

## Quickstart

To run the program, you simply provide the source file you want analyzed:

```bash
sbc test_file.c
```

which will then give you an output like the following:

```text
Number of statements: 10
Number of blocks: 4
```

You can also specify how many threads you want to use by specifying the `--threads` parameter:

```bash
sbc --threads 7 test_file.c
```


