## A simple brute force break password in zip encripted files.

It's necessary use 4 objects when calling this function (unless you are calling with the flag --help).
The first is min_leters_quantity, which means the min quantity of letters in the password.
The second is max_letters_quantity, which means the max quatity of letters in the password.
The third is letters, which means is the of letters used to build the password.
The fourth is path_to_archive, which means that is the path to the encripted zip archive.

Note that the min_letters_quantity must to be at least 1 and the max_letters_quantity must to be 
equal to or greater than min_letters_quantity.

Example: ./bin 1 8 "abcdew" "/home/name/my archive.zip"
