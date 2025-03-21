# Good Sources
- https://web.archive.org/web/20110219155647/http://howtohft.wordpress.com/author/howtohft/
- https://www.youtube.com/watch?v=b1e4t2k2KJY
- https://www.youtube.com/watch?v=nmYx6tQxtSs
- https://louisponet.github.io/blog/posts/icc-1-seqlock/

# First Ideas

- Try to have a single thread to manage the order book
- Possibly will be quite slow, then try to multithread the order book
- Have a thread or two to be producers to the thread handling the order book