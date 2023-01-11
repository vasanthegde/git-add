# git-add
It is simple CLI tool for your **_git add_** command.

there are great tools like lazygit or gitui which provides so far better fetaures for git commands. 
If you are looknig for full blown git integration please go to such tools.

then why this yet another tool ?
I am simple man and most of the time I use following git workflow in terminal
1. git status 
2. git add
3. git commit
4. git push

the main problem with "git add" command is it need complete file path which is bit annoying. 
Hence this tiny CLI wrapper which takes shorthand or simple file names as input and executes git add command to matching unstaged files. 
No need to remember any keyboard shortcuts. 

# Guide

### Usage
```
./git-add
./git-add lib.rs                      // match src/lib.rs
./git-add MGAF                       // match file with name MyGlobalAbstractFactory.java
./git-add MGAF lib.rs main.rs        // match multiple names
```

![git-add1](https://user-images.githubusercontent.com/12895102/211737721-5a2b8725-345c-4344-b0a2-271788738e56.gif)
![tty](https://user-images.githubusercontent.com/12895102/211737830-3dfbbc2c-6723-4aba-b070-bdcd517a80df.gif)

## Installation
If you would like to build from source:

```
git clone https://github.com/vasanthegde/git-add
cd git-add
cargo install --path .
```
else checkout release section to grab available binaries.


## Upcoming features
1. performance optimazation by using hashmap look ups
2. color coded user prompts ( green for added files, yellow for modified and red for deleted files ) 
3. perhaps more file matching pattern
