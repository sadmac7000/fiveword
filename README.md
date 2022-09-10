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

The solution here is a hybrid approach. We assemble a tree where each node contains in effect a set
of letters corresponding to a single word. We insert each word into the tree as follows
1) Insert a new node as the child of the root.
2) For each previously existing child of the root, if it has no letters in common with the new word,
   insert into the subtree rooted at that child.

Once assembled, every path from the root to a leaf is a solution to our problem.

The second optimization we do is in the comparison of words themselves. Matt reduces words to python
`set`s of characters and tests commonality by merging the sets then checking the cardinality of the
result. Instead, we map each word to a 32-bit unsigned integer, where each bit in the integer is
non-zero if an only if the letter of the alphabet sharing that bit's index is present in the word.
So bit 0 is set if the word contains the letter 'a', bit 1 if it contains the letter 'b', etc. At a
basic level this is simply a highly machine-optimized representation of a set of characters, meaning each node in the tree is only an 

Finally, there is the obvious lower-level optimization: both Matt and Benjamin's solutions are in
Python, this solution is in Rust.

Annecdotal timings on laptops have been the standard thusfar. For me this runs in under forty
seconds.  That's running as `cargo run --release word_alpha.txt`
