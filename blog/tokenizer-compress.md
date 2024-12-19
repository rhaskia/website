---
description: A small compression project I tried to make
published: 2024-04-20
tags:
  - compression
  - rust
---
# Text Compression 
I'm sure you've heard of Zipf's Law, the one where it states that 20% of words make up 80% of language. 

This has always had me wondering if I could make use of it to compress text or something of the likes - you could store a lot of text in a much smaller amount of space than regular ascii or Unicode. 

# How Well Does Normal Text Storage Do?
The average word in English uses anywhere from 5-6 characters (with spaces), and though an ascii character usually takes up 8 bits of storage, you could squeeze it into 5 bits if you were to force  it into lowercase and clean any punctuation out of it. This would leave you with something between 25-30 bits per word, a decent size smaller than the 40-48 bits that you would normally have. Of course this would lose you a decent bit of information, but it wouldn't be hard to supplement anything lost with an escape character or two. If anything punctuation would be a nuisance, but you might be able to fit the more necessary into that 32 bits. 

# Zipf's Law
To go a step further, I'm going to try to treat each word as a unit rather than characters. This definitely has it's setbacks, but I'm going to ignore those at the moment and see what I can do. 
The list of words I have to start with is  from the website https://www.wordfrequency.info/files.asp, and numbers 5050 in total. The closest I could get to this in bits would be 4096, but I don't really want to scrape anything off of this so I'll go for 13 bits with 8192 combinations. Of course I would love to use a much larger dictionary, but this is as close as I can get. If I were to use a much larger dictionary though, I could fit the (apparently) 1 million English words into 20 bits comfortably. 

To start off with, I'm going to be testing my compressor on the Lewis Carrol's Alice in Wonderland, mostly because it's public domain and easy to access from the Gutenberg archive. I began work on splitting this into assorted symbols and words, and then cleaning those words into either an `Unknown` data type or one for an index of the list. This took a lot longer than I thought it would, as I screwed up splitting the text at least four separate times. 

I ended up switching to a different database while testing, as 5000 words wasn't doing it for me. For reference this is the database I was using: https://www.kaggle.com/datasets/wheelercode/english-word-frequency-list

Finally got this working, and I'm getting some interesting numbers coming through. With just 10 words loaded in, 6,200 of the total 29,674 (20.8%) words inside the txt file are recognized (insane for just 10 words), with 50 it almost doubles to 11,112 out 29,674 (), at 500 it hits 18,565 recognized words, or 62%. At 5,000 it hits 84%; at 50,000 it sits just around 97.6%. Making the full use of this roughly 9 million in size database, I can squeeze out 99.8%, or 64 unknown words out of nearly 30,000. The majority of these 64 seem to be a parsing error rather than actually being unknown, including things like "importantunimportantunimportantimportant", "meanstomakeanythingprettier", "arithmeticambition", and other words meant to be combinations of many (I figured out that these are due to the `Em Dash`, a unicode character similar to a wider hyphen. Fixing the code to work with unicode brings the 5000 word list up 1.8%, and presumably the rest up a decent margin as well).  There are some genuinely unknown "words" in the 64 though, such as "soooop", "eeevening" and "beauootiful". These are generally unavoidable though; I'll have to make my end program deal with non-standard words.

# The Hard Part: De/Serializing
This is a bit more difficult, as if I'm going for full performance I'll be playing with bits. I ended up making a bunch of messes trying to get this to work, but in the end I got it working (Serializing at least), compressing the text down to about half the size. A simple deserializer was a bit harder, but with a bit of code and some state management I managed to get it to read the majority of the compressed text well. I did find out partway through making it that the compressor was screwing up by saving words in little endian format instead of big endian format, but that was an easy fix. The next issue I came across was the program breaking when it hit the control character, devouring everything in it's path into one massive sludge.
![[Pasted image 20240417102631.png]]
This was a really easy fix, as it turns out I was not clearing a string I was using to store the escaped text whenever the escape part ended. After that was fixed, my basic test example run well

# Huffman Coding
This is where the Zipf's law really comes in to squeeze more compression in. With the database I have, it also contains a matching frequency number ('the' has a frequency of 53,135,851,162, 'compression' a frequency of 10,712,155 and 'Huffman' a frequency of 10712155). Huffman Coding is a way to take advantage of that, and works similarly to Morse code where more common letters have shorter codes, and less common have longer codes, i.e. e being a single dot, and z being two dashes and two dots. This system wasn't exactly perfect though, needing little spaces between letters, something that while worked fine for the analog systems that were around commonly during the days of Morse code. Huffman encoding, made to work with digital systems while still taking advantage of variable code length, uses unique prefixes to achieve this. With these unique prefixes, reading a code is as easy as going through each consecutive bit until you hit a match, as no code starts with another, allowing you to be certain you hit a code. The specific details on how it works is a bit confusing, I would recommend reading the [Wikipedia page](https://en.wikipedia.org/wiki/Huffman_coding), or this [article](https://eclecticlight.co/2015/10/22/reinventing-morse-code-using-modern-theory/) on it.

Anyway, I managed to get a python script running to sort through the database; loading in all the frequencies and words and feeding them into the algorithm. Out came a nice big list of the first $2^{16}$ words and their responding Huffman codes. Some interesting ones include the, at only 5 bits (used to be 4 before something I'll mention ahead): 10110, that at 7 bits, with all of them 0. I loaded these codes up into my compressor script and messed around with it until it managed to replace each word rank with a Huffman code instead. Running Alice in Wonderland through it, I managed to squeeze out about 45 kb from the original 171kb, a decrease of roughly 60% percent. Testing on some other documents, I managed to get roughly the same compression amounts, except for one technical document that I squeezed down to 95kb from 417kb, 23% of the original size. I expect that with some tweaks, I could probably get the algorithm to do similar compression sizes on other documents as well, possibly with a preprocessor that creates a frequency list of the words, punctuation, etc. inside the text document that it's working with.

# Future Ideas
I'm not sure how much more I could actually squeeze out of this project. Of course I could make it so it creates a word list based off of itself, but that likely wouldn't do too much to the file sizes that it creates. A really ambitious idea that I did have was having each word not be scaled on the frequency of it across all of English, but just the frequency of it compared to the word before it; context would hopefully let me use less and less bits. That idea does however requires knowing the frequency that words appear in front of each other, which would be nigh impossible across all of English, though possible with a defined set of words or just using characters.

If by any chance you're interested in the code I used to do this, it's all at https://github.com/rhaskia/tokenizer-compress.