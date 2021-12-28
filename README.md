# Rust Keys


```
 ______               __     __  __                    
|   __ \.--.--.-----.|  |_  |  |/  |.-----.--.--.-----.
|      <|  |  |__ --||   _| |     < |  -__|  |  |__ --|
|___|__||_____|_____||____| |__|\__||_____|___  |_____|
                                          |_____|      
```

## BigRandom

BigRandom, or `bigrandom` creates a base 36 random number from a vector of random u32 ints.  The default count is 8 that creates a large random number but can be expanded with `--size` flag to any desired size.

Example:

```
bigrandom --help 

BigRandom 21.12.28
darryl.west@raincitysoftware.com
A big, potentially huge random number generator to create base 36 random keys

USAGE:
    bigrandom [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --base <base>    set the radix base between 2 and 36, defaults to 36
    -s, --size <size>    set the number of random u32 words to generate, defaults to 8.


bigrandom

2wk4b9srzggjmi9ulhxdmju2rlcq6750gug4pnz04jeesmiduk

bigrandom --size 16

e7jmzl95p42judq54wg0ys9inuacadtch77qlgb4cxu8b3c7o3zelw6tuzegbt9cww8e38r5qr8bscnj7g6e6fut87st7cij83u

bigrandom --base 2

110100111100110011010011011101010111100001110000001010011111001001110011101001110000010010011010101110111110101100110110000101100000001100001001000101111101110011011000100011001011010100000100111011110100000111101111110100010101110110110101000100000001100

```

###### darryl.west | 2021.12.28
