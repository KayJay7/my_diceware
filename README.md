# my_diceware

`my_diceware` is a simple (and inherently useless) implementation of the diceware password generation algorithm. It's more of a fun project than anything useful.
I said this is inherently useless because you don't want your password to popup on your terminal when you are choosing one. But if you are by yourself and close the terminal right after choosing, go on.

The original algorithm, by Arnold Reinhold, required to roll 5 six-faced dice to choose a word from a list of 7776 (6^5) words, and repeat the process untill you had enough words for a secure password.
Each word adds adds 12.9 bits of entropy, so you would need 5 words to get around 64 bits of entropy, and 10 to get around 128 bits. The reccomended count is 6 words (77.5 bits), so it's set as default. Adding a couple of symbols in random places in the string (not between words, inside words) will make the password much more secure, even using just 4 words.
Of course, this program will not generate 5 random numbers for every word, but just a single number between 0 and 7775.

It's important to notice that the wordlist is not just any wordlist, the words were choosen to be "mor random", so you need to make sure you are using a good list.
The one provided with the algorithm and found (just for reference) in the "assets" directory is the original list from Reinhold, but there are others in different languages and from different authors.

## Why is this interesting?

Ignoring the "popping up on terminal" problem (and no, sending the passwords to the clipboard doesn't solve anything, it makes it easier for other programs to read the password). I decided to focus on the "use this specific wordlist" issue.
It's easy for anyone with writing rights for the executable to find the word dictionary (all the static strings in a program are stored concatenated in plain text in the same area) and alter is.
If an attacker were to replace all the strings with `"a\0\0.."` the program would always display `a a a ...` as generated password, making the cracking of the password just a matter of guessing the number of `a`s.

To mitigate this problem, before generating the password, the program generates a salted hash of the whole dictionary and compare it to an hash that is hardcoded has an array of bytes.
The hash is already more difficult to find compared to the strings, but to spice things up, it's also encrypted the hash with a stupid streamcipher I came up with in 3 minutes.
Despite sounding expecially stupid, all of this actually works, try it for yourself.

This is not 100% secure, an attacker could raplace the encrypted hash, replace it, and the program wouldn't notice. This, however, is definitely more difficult.
There's probably no way to make it 100% secure. Even if I where to use asymmetric encryption to ecnrypt the hash, so that the attacker wouldn't be able to encrypt their hash, the attacker could just replace the public key.

## This is stupid, why would I care?

Yes, it is stupid.

It's intended as an "experiment" or proof of concept. I wanted to try tinkering around with hashes.
Of fourse, the idea of encrypting hashes to verify integrity is not something I invented, this is an occasion to see how you shouldn't do it like i did.
In particular, why you shouldn't use stream ciphers for this: if you xor the hash with the encrypted hash you extract the entire keystream, and the hash is not a secret.

Also there is a big bonus more malicious flaw I intentionally didn't point out, can you find it yourself?

## Any future developement?

Probably not, maybe one day I will try to make a version of this that actually make sense. For now you can use it to generate diceware passwords an you are almost sure that the actual dictionary will be used.
