This is an attempt to one-up Matt Parker's programming skills as demonstrated in
[Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)

Matt's problem is to find all lists of five English words with 25 distinct characters, given an
input dictionary.

Matt's solution, by his report, took several months to run. From his description, it tested every
word against every other word to generate pairs of words with disjoint sets of characters, then
compaired every pair with every pair to generate lists of length four, then checked every word once
more to extend the lists to length five. By Matt's report this solution took literal months to
execute.

A subscriber named Benjamin submitted an [alternate solution](https://gitlab.com/bpaassen/five_clique)
which instead constructs a graph from the word list where each word is a node and two nodes are
connected if they share no common letters. The algorithm then locates 5-cliques within the graph to
discover solutions. Matt's annecdotal test of this program said it ran in about 15 minutes on the
same data set.

The solution here resembles Matt's solution more closely than Benjamin's. For each item in the list
of words, we scan the remaining words to see if we can find a word with no letters in common. Then
we scan the words *to the right of that word* to see if we can find a third word which has no letter
in common with either of the first two. For each such word we find, we scan the words *to the right
of that word* to find a fourth disjoint word, and so on for the fifth. This exploitation of the
order of the list means each subsequent word added to a given solution can be scanned from a smaller
list than the previous, reducing the time complexity of the search. This avoids inefficiency in
the second step of Matt's algorithm, where two pairs might share an entire word in common but are
still compared in the second pass. This should be a better optimization over the naive n^5 solution
than Matt's pairing system.

The second optimization we do is in the comparison of words themselves. Matt reduces words to python
`set`s of characters and tests commonality by merging the sets then checking the cardinality of the
result. Instead, we map each word to a 32-bit unsigned integer, where each bit in the integer is
non-zero if an only if the letter of the alphabet sharing that bit's index is present in the word.
So bit 0 is set if the word contains the letter 'a', bit 1 if it contains the letter 'b', etc. At a
basic level this is simply a highly machine-optimized representation of a set of characters, but we
further enhance our algorithm by storing our list of words as a B-Tree mapping from integer keys
describing the set of characters as shown to the words themselves. When we test a given word, we can
use a few bitwise operations to calculate the smallest possible next value that might map to a word
meeting our criteria, and use the B-Tree structure to skip ahead to that value.

Finally, there are lower-level optimizations; both Matt and Benjamin's solutions are in Python, this
solution is in Rust, and both Benjamin and (as far as we know) Matt's solutions are single-threaded.
This solution is parallelized.

Annecdotal timings on laptops have been the standard thusfar. For me this runs in under two and a
half minutes. That's running as `cargo run --release word_alpha.txt`

There may still be optimization to be had here. For example, if `a b c d e` and `a c d e f` are both
correct results, the determination that `a b` is disjoint from `c` for the first result will not be
reused to determine `a` is disjoint from `c` when materializing the second result. A more
complicated data structure while iterating might make this possible, trading a bit of memory for
some comparisons, though there may be some difficulty avoiding allocations to manage this more
complex structure.
