# ModMark
## A work in progress markup language that promotes easy source attribution, variables, templates, and full static site/web publishing capabilities

ModMark is a heavily opinionated markup language inspired by John Gruber's Markdown. Created to streamline complex document and blog post creation, ModMark features templates, variables, and and macros to make creating common elements and tag layouts found in HTML a breeze. ModMark exports to HTML5, using CSS3 and little to no JavaScript so your sites stay light, beginner friendly, and highly customizable. ModMark can be used as either a complete site markup language, or as a step in your site's creation process. The end goal for this project is to allow ModMark to become easily-intigratable serverside or in static site generators. 

# Syntax

Much of ModMark’s syntax is borrowed from Markdown. If you’re already familiar with general syntax you already know ModMark basics. 

If you are unfamiliar with Markdown, you can learn it quickly at https://commonmark.org. 

# Plans:

Currently, ModMark supports full CommonMark syntax, and can be downloaded as binary and use immediatly (Please check below for install instructions). Eventually, I plan to add support for the following:

1. Variables
2. Templates
3. Attributes
4. Macros
5. Active Source Attribution

## Distant Plans
Eventually, I'd like to grow ModMark into a more complete suite of web solutions for anyone looking to make a blog, small store, or portfolio website. Some elements of creating the application align well with the following goals:
1. Dynamic Web Application Creation
2. Ability to create and manage a micro store using Modules and payment gateway intigrations (Stripe, Square, etc...)


# Install or Compile
## Install
If you're not interested in compiling the binary or development, you can get it directly [here](https://drive.google.com/file/d/14WBnhxSWnE9uqZJ-3Fg13e3Kh-Zzw08Z/view?usp=sharing)(Google Drive) or through Cargo (*working on it*).

After downloading the binary, move it to your home directory like so:
```
$ mv ~/Downloads/modmark ~/modmark
$ cd
```
Then, make it an executable.
```
$ chmod +x modmark
```
Then just run the app:
`$ ./modmark`

## Compile
Modmark is built soley in Rust, and thusly requires the Rust Language and Cargo installed on your machine. You can fully install the langauge, Cargo, and the entire rust toolchain by installing rustup. 

Please follow the guide at https://www.rust-lang.org/tools/install if you are unfamiliar with Rust.

After downloading the Rust toolchain, '$cd' into the ModMark directory (assuming you've cloned the repo from GitHub) and run the following:

```
$ cargo build
```

It's highly reccomended that if you're interested in developing on top of ModMark, or if you want the most up-to-date version of the binary, that you familiarize yourself with Cargo. You can learn more about Cargo here: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html. This is not a beginner friendly method of running the application, and I am currently looking for additional solutions to make ModMark as easy as possible to use, including a front-end and web server intigration to make using the project as easy as possible for as many people as possible.

# Suggestions and Issues
All code issues should be called out using GitHub's built in Issues feature. Suggestions, feedback, or anything fun you just want to share should be shared with me on Telegram: https://t.me/TristanIsham.

# FIN
Thank you for giving ModMark a try! I really appreciate it! :) 




<!--
## Unique Syntax
### WiP
 ### Variables 
Are you ever writing a story and after 2,000 words realize that you’ve been misspelling your main source’s employer? Have you ever published a post only to realize that you mistyped a name in one or two places?

Sure, these errors could be fixed through keyboard shortcuts and manual labor, but ctrl+f is notoriously finicky, and nobody wants to have to crawl through even more text right before their deadline. That’s where ModMark’s variable system comes in. 

You can declare a variable at the beginning of your document like so:

<{foo}[bar]>

Whereas “foo” is the variable and “bar” is the content. In Rust or JavaScript this would look like:

let foo = bar;

You can call your variable anywhere in the document with this syntax:

<{foo}/>

A working example of this could look like:

<{name}[Reggie]>

//—Snip—

My source, <{name}/> is a great grandmother! -->
