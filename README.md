# On Hold

This is currently on hold.

I hit a major barrier trying to change a sprite's transparency at runtime. Documentation is really light and, even though the community is generally welcoming and helpful, couldn't find anyone who actually knew how to do it for over a week. Tried using several components and combinations, including Taint, Transparent, and some others pointed at discord, but couldn't get it to work. 

Later on I realized it isn't just dynamic alpha that didn't work, but trying to use a png with partially transparent pixels (not 100% transparent) also doesn't render correctly.

This is the closer my research got me, helped me understand better how Amethyst's rendering works, but not what I needed in the end:
https://github.com/amethyst/amethyst/blob/7ed8432d8eef2b2727d0c4188b91e5823ae03548/examples/sprites_ordered/main.rs#L463-L482

I then started to research how to get a hold of the `glx/vulkan` context just so I could make a pulse-like effect with transparency. I stopped after realizing how retarded doing that would be and how it would open the door for a lot more of these "workarounds" in the future.

Now I need to decide whether I want to keep digging and hope to eventually find out how to do dynamic sprite transparency or if I should simply give up on this, admit defeat and reduce **a lot** the intended scope and polish of this project (what is honestly much more likely at this point). This also means placeholder assets will probably be final.

If you know how one can achieve dynamic transparency of sprites with amethyst, I would really appreciate the help. Feel free to drop me a message, a gist, a PR, a Docs link.

# WIP

This is still a work in progress

Follow the progress at https://trello.com/b/iFWbsMax/space-shooter

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run --release
```

### License

This work is licensed under GPL-3.0, except the assets folder that has ATTRIBUTION or LICENSE files wherever necessary.