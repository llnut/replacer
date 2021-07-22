# replacer

A tool that replaces the content of a source file with new content with different weights, written in Tokio

## Usage

We assume that there is a file named `s.txt`.

```
jcsora@jcsora-g14 rust/replacer (master *%) » ls
s.txt  Cargo.lock  Cargo.toml  Config.toml  LICENSE  README.md  src  target
```

Its contents are:

```
https://img.aaa.com/blog/2021/avatar/1.jpg
https://img.aaa.com/blog/2021/avatar/2.jpg
https://img.aaa.com/blog/2021/avatar/3.jpg
https://img.aaa.com/blog/2021/avatar/4.jpg
https://img.aaa.com/blog/2021/avatar/5.jpg
https://img.aaa.com/blog/2021/avatar/6.jpg
https://img.aaa.com/blog/2021/avatar/7.jpg
```

Let's create a new Config.toml and fill in the following:

```
pat = "https://img.aaa.com/"

to = [
    ["https://img1.a.com/", 20],
    ["https://img2.a.com/", 20],
    ["https://img3.a.com/", 20],
    ["https://img4.a.com/", 20],
    ["https://img5.a.com/", 20],
]
```

Then we run our replace using `cargo run s.txt`. After it, cat the newly created file to see what happens to its contents:

```
jcsora@jcsora-g14 rust/replacer (master *%) » cargo run s.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/my-replacer s.txt`
jcsora@jcsora-g14 rust/replacer (master *%) » ls
Cargo.lock  Cargo.toml  Config.toml  LICENSE  r_1626948159259  README.md  src  s.txt  target
jcsora@jcsora-g14 rust/replacer (master *%) » cat r_1626948159259
https://img1.a.com/blog/2021/avatar/1.jpg
https://img2.a.com/blog/2021/avatar/2.jpg
https://img3.a.com/blog/2021/avatar/3.jpg
https://img4.a.com/blog/2021/avatar/4.jpg
https://img5.a.com/blog/2021/avatar/5.jpg
https://img1.a.com/blog/2021/avatar/6.jpg
https://img2.a.com/blog/2021/avatar/7.jpg
```

As you can see, their original content is replaced with other content with a certain weight. Use this tool to replace the contents of your SQL files when you want to do load balancing based on URL parsing. And maybe it has other functions. Welcome to use it. ^_^

