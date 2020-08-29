Markup language that promotes easy source attribution, variables, templates, and full static site/web publishing capabilities

ModMark is a heavily opinionated markup language inspired by John Gruber's Markdown. Created to streamline complex document and blog post creation, ModMark features templates, variables, and and macros to make creating common elements and tag layouts found in HTML a breeze. ModMark exports to HTML5, using CSS3 and little to no JavaScript so your sites stay light, beginner friendly, and highly customizable. ModMark can be used as either a complete site markup language, or as a step in your site's creation process. The end goal for this project is to allow ModMark to become easily-intigratable serverside or in static site generators. 

# Syntax

Much of ModMark’s syntax is borrowed from Markdown. If you’re already familiar with general syntax you already know ModMark basics. 

If you are unfamiliar with Markdown, you can learn it quickly at https://commonmark.org. 

## Unique Syntax
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

My source, <{name}/> is a great grandmother!
