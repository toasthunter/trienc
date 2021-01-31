# Trinary pixel text encoder written in Rust

This program takes a text, encodes it into trinary numbers, and displays the letters in 3-pixel columns where red is 0, green is 1, and blue is 2.

Only the english letters of the alphabet and spaces are valid characters, everything else is invalid, and the program will not work with those.

The encoder reads the text from the `input.txt` file. If this file is not present in the executable directory, the program will not work.

