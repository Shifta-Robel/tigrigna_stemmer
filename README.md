# Stemmer for the Tigrigna language

Implementation of stemming algorithm based on the paper '***Development of stemming algorithm for tigrigna text,Yonas Fisseha, June 2011***', but with slight deviations, the compiled list of affixes was sourced from another such [project](https://github.com/luelhagos/Stemming-for-Tigrinya).

## Algorithm
  1. *Tokenization*: Transforms input string into a vector of tokens, a token being a sequence of only geez letter characters, The only characters taken as valid are those within the unicode geez range i.e. 1200 - 135A, all else are parsed as whitespaces, and all stopwords get ignored.
  2. *Adjecent character pair deduplication*: string gets analyzed in it's *sadis form*, if pair of letters are duplicated the first pair gets removed
  3. *Prefix-suffix pair stripping*
  4. *Prefix stripping*
  5. *Suffix stripping*
  6. *Ajjecent character deduplication*: Same as step 2, but with single letters

## Deviations from paper
  - All processing is done on the original geez form of the text, romanization seemed redundant since deduplication could be done easily of the sadis for of the word, and counting radicals for most of the cases seen was just the count of the geez characters in the word.
