## A tool for cracking the password of a protected zip file

It's recomended to read the manual (/src/Manuals/man.txt) to understand how to
use this program.

## Description
This project features more than one method for cracking zip file
passwords. Currently, 3 methods are support: the first is a 
simple brute-force cracker through the creation of anagrams using several
user-defined letters; the second is via a dictionary attack, where the
tested passwords must be in a .txt file with each entry separated by a
newline; the third method combines the two previous approaches: besides
testing each password individually, it also tests possible anagrams
formed by those passwords.

Note that this program doesn't have support for GPU acceleration and 
multi-thread, and I don't have the plan to add it my self.
