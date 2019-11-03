Torrent File
====================================

I wanted to make a quick program that would allow me to add GetRight-style web seeds to torrent files. I looked around at the state of crates and didn't find one that could do it. Notable places I checked (that are better than what's here):

* [bip-metainfo](https://crates.io/crates/bip_metainfo) (No GetRight seeds)
* [torrentinfo](https://crates.io/crates/torrentinfo) (Essentially thing as this guy)
* [serde_bencode](https://crates.io/crates/serde_bencode) (Where me and torrentinfo got the torrent example)

If I'd spent a little time looking and had found 'torrentinfo' earlier, I would have forked it and added the web seeds to it. I may still! But for now the `web_seeds` binary works. This one's also Apache/MIT licensed instead of GPL3 🤷‍