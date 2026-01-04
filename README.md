 > We're dsplce.co, check out our work on [github.com/dsplce-co](https://github.com/dsplce-co) 🖤

# cant-happen

🙅‍♂️A library and manifesto on panicking gracefully and only if something _can't happen_.

⸻

## 🧪 Usage

Use `cant_happen` in lieu of `expect`/`unwrap` on a `Result` or `Option` you are certain _cannot fail_. Pass the reason why as the method's argument. Think of what you write in the argument as the continuation of the sentence "A failure _can't happen_ here because...".

```rust
use cant_happen::prelude::*;

let regex = Regex::new("[a-z]").cant_happen("regular expression is hardcoded");
```

### Set GitHub issue creation link

For public projects, you probably want to get your users to report the occurrences of such "impossible" failures as soon as they happen. Make it extra easy for them and set up your GitHub repository's issue creation URL in `cant-happen`. The library will provide them a link to create an issue with the error details pre-filled.

```rust
use cant_happen::prelude::*;

fn main() {
    set_repository_new_issue_url("https://github.com/<your_name>/<your_crate>/issues/new");
}
```

⸻

## 💡 Motivation

Not sure if you've ever wondered about it, but the entire exception-handling system Rust is based on is a mechanism of enforced error handling. Have a look:

```rust
fn main() {
    std::env::current_dir(); // It's `Result<PathBuf>` so you basically cannot access the path unless you handle the output, the type you get is `Result` (containing `PathBuf`) but not `PathBuf`
}
```

So you're forced to handle it like this:

```rust
fn main() {
    std::env::current_dir(); // `Result<PathBuf>`

    let Some(path) = std::env::current_dir().ok() else {
        eprintln!("This CLI operates on the data connected to the current directory and you seem to have run it in a standard environment that does not yield one");
        exit(1);
    };

    // Do some further processing with `path`
}
```

or like this:

```rust
fn main() {
    std::env::current_dir(); // Result<PathBuf>

    let path = std::env::current_dir().unwrap_or(PathBuf::new("/"));
}
```

If you think about it, it's actually really great compared to some languages that don't tell you that sth is throwable at all. For example in JavaScript (the 2nd language I use):

```javascript
const userInput = prompt();

let element = document.querySelector(userInput);

if (!element) {
    alert("No element found");
} else {
    alert(`Found an element`);
}
```

You might think this code is properly-secured, but it's not. `document.querySelector` doesn't just return `null` or `HTMLElement` depending on whether the given element is present. It throws. Eg. when the input is not a valid selector.

The thing is. You don't know it. With any function. (Until you make an enormous effort to chase the source code of every single function and all the functions it calls, respect for *you* if you do *that*). It's kinda okay if it comes to the standard library of a language as you can memorise it, but nowadays we all utilise new libraries and API-s daily.

We can then come to a conclusion that for many languages everything you call is actually throwable and for every such case you might not know it until you find that one user who will face this error — which may destroy their ~~life~~ experience.

But let's be serious, imagine going to a café, ordering a coffee and not knowing if you'll actually get it as you don't know if the barista will die halfway in the process or quit. Many languages are exactly like that.

The barista might have died (like a script's runtime) because you ordered sth custom, in the real world they should either adhere to your request or tell you they don't serve it. When you hire a barista you trust them that they're reasonable enough not to _panic_ in a front of the customer and instead gracefully handle the _exception_. I require the same from the functions I call.

Ofc, even when someone can't handle the unknown you can create a policy for your staff which could go like:

```javascript
while (true) {
  let coffee = null;

  try {
    coffee = cafe.makeCoffee(); // automatically assigns a barista to all not "owned" and unfinished coffees
  } catch (error) {
    if error === `BARISTA_DIED` {
      cafe.hireNewBarista();
      continue;
    }

    // not to mention all the other exceptions we should handle
  }

  return coffee;
}
```

The problem is that you only reflect to add this `try-catch` to your "policy" after the first time a barista panicked. It's too late. You've lost a customer and they'll give you zero stars on Yelp.

Rust on the other hand does its best to inform you what could go wrong before you even hire it. But when you're not patient enough it allows you to enforce the behaviour from other languages too — by using `unwrap`s; whenever you run it on a Result or Option the program will panic (in case of an error) just as if you hadn't written the `try-catch` in JavaScript. But you knew it already, it's you who silenced the great employee.

But using `unwrap`s you basically opt out from all of these benefits.

### Can we not use `unwrap`s at all?

You could very well think that and this time I **could** possibly comment so let's consider that case:

```rust
fn main() {
    let regex = Regex::new("[a-z]").unwrap();
}
```

You can see the `unwrap` right? And you're wondering when the `Regex::new` can throw then? The answer is, when the regex is invalid. But we see it's perfectly valid, right? It's just one letter from `a` to `z`.

So obviously the `Regex::new` accepts dynamic input as well, if it's dynamic you don't know the value at build time, you only know it at runtime so you can't know if it's valid and hence you should handle the error properly.
So obviously the `Regex::new` accepts dynamic input as well. If it's dynamic you don't know the value at build time, you only know it at runtime, so you cannot _tell_ if the input is valid at the stage of writing the code and hence you should handle the error properly.

That's if the input was dynamic, but it's not.

What to do then?

Okay, we can leave it as it is and _remember_ that this particular `unwrap` is safe.

The problem is that this way you give yourself permission to use `unwrap`s 'cause you think "sometimes they do make sense", stopping considering every case thoroughly from now on.

Well, the thing is you can forget about it. Somebody else might not be aware of your motives. What's more, let's say a new team member joins the project and sees other are using `unwrap`s, they might think it's okay to use them too, unaware you had a good reason that _one_ time. Finally, at the point when you want to make your codebase more bulletproof and want to get rid of all `unwrap`s you realise that some of them are needed and then your entire clarity is lost.

### Zero-unwrap policy

Everything would be simpler if we weren't using `unwrap`s at all. Imagine it, with Rust, no `unwrap`s means almost no room for an error (excluding some edge cases).

But we won't be handling such things:

```rust
fn main() {
    let Some(regex) = Regex::new("[a-z]").ok() else {
        eprintln!("Regex is invalid");
        exit(1);
    }

    // use your lovely `regex` variable here
}
```

It's just stupid and makes everything more complex, why make code complex because of a case that can't happen.

We basically need to say — that can't happen, and I mean it because of XYZ. XYZ *in this example* is *the regex is hardcoded*.

This pushes us into outlining sth like:

```rust
fn main() {
    let regex = Regex::new("[a-z]").cant_happen("the regex is hardcoded");
}
```

This way:
- Let's say you ran into this code and you see that someone had resolved `Result` this way
- It's like the `unwrap` but they gave justification
- If you want to modify this code you kinda feel obligated to update the justification if it needs to be updated or remove the `cant_happen` call if that's what is suitable given your change

What's more, we can actually go one step further and add a message just in case it indeed fails (even though you should never use this if you can see room for that).

### How is this different from `expect`

It's actually very similar to `expect` and the name of this lib was even supposed to be "better-expect" at first but:
- If you add this lib to your project you should consider yourself not using `unwrap`s anymore, if it comes to `expect` people normally use it next to `unwrap`s, because nobody ever defined a policy to not do it
- `expect` is just as `unwrap`, if you equip people with it they will basically use it everywhere, or almost everywhere depending on how often they are tired which is also because…
- Nobody ever told anybody when to actually use `expect`. I tell you when to use `cant_happen` — if you're 100% fucking sure it won't panic. Knowing sth for 100% is (in practice) usually 99 or 90 or 80 percent. You can improve your percent the more experienced you are but be prepared a for case when you're wrong
- If you opt in to the GitHub issue template, you increase the chances sb raises it

In other words, at the end of the day, you shouldn't be using `unwrap` and/or `expect` and even `cant_happen` until it totally makes sense in 1% scenarios and this lib will help you take control over this entire landscape. It's all about clarity, aiming for zero unwraps should make you more mindful when using `cant_happen` (which is another `unwrap`, but with a more narrow purpose set — it basically tries to force you to use it only in cases that unwrap/expect were originally designed for [but nobody seemed to get it])

### Outro

Whatever you think at this point, what I can be certain about (more than you about your unwraps) is that:
- Even a single `unwrap` tells your team members you use `unwrap`s as a team
- Even a single *sensible* `unwrap` makes it harder to notice production-breaking `unwrap`s because your general opinion leans to be that it's ok to `unwrap` sometimes
- Whenever you say "sometimes" about a thing it starts to become unmeasureable
- The `expect` doesn't hold anymore, people aren't using it in different cases than `unwrap` but as a replacement, so their code keeps panicking as much as it did in the first place

Finally…

My message isn't let's now replace all the unwraps with `cant_happen`s, (like you could be suggested to for `expect`s). My message is go through all of your `unwrap`s and replace only the ones that you're 100% sure **can't happen**. The unwraps that remain are ticking bombs. Which you've discovered thanks to `cant_happen`, just like you could discover further mines in minesweeper thanks to its flagging feature. `cant_happen` is mainly a psychological tool that helps you surface the actual unhandled errors.

Last but not least…

I really hope that `cant_happen` won't meet the same fate as `expect` and people won't just start using it instead of `unwrap` :) You use `unwrap` too often and this lib is another `unwrap` that is just supposed to help you do it more rarely.

⸻

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cant-happen = "0.1"
```

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/cant-happen](https://github.com/dsplce-co/cant-happen)<br>
📦 **Crate**: [https://crates.io/crates/cant-happen](https://crates.io/crates/cant-happen)

PRs welcome, feel free to contribute

⸻

## 📄 License

MIT or Apache-2.0, at your option.
