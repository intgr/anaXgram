# Ana*gram

Finds anagrams from a text dictionary file. For Helmes's competition at https://www.helmes.com/careers/challenge/

The code is quite ugly but who cares, it only needs to be fast. :)

## Usage

1. Install rust & cargo
2. Build:
	```
	cargo build --release
	```
3. Download and unpack lemmad.txt from http://www.eki.ee/tarkvara/wordlist/lemmad.zip
4. Run the command:
	```
	./target/release/analgram lemmad.txt era
	999,aer,are,era,rae,rea
	```

\o/

## Authors

Marti Raudsepp <marti@juffo.org>
