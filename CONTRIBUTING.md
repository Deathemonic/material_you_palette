# Contributing to material_you_palette

This is the first project I've pushed (publicly) where there is a reasonable expectation that anyone might want to contribute. This is not my first project ever, but all of my work to date has mostly been just hacking at things and trying to learn something new. That being said, I'm not entirely sure where to start other than by first, thanking you for taking the time to contribute or at least consider it! :tada: Hopefully this project will get a little more attention than just passing curiosity and together we can make it better and more useful. Should this take wings, I have zero intentions of running it entirely by myself and certainly not as a dictator. I am open to suggestions for improvements and learning new things. So, we'll start this off (hopefully on a good foot) with some really basic guidelines and grow from there.

## Table of Contents
1. [Basic Rules (a.k.a. Code of Conduct?)](#rules)
2. [Project Goals](#goals)
3. [How can I contribute?](#howcani)


### Basic Rules<a id="rules"></a>

First some basic rules that I have for my all my dev teams. They have served me and my teams very well for years. They just might serve well here.

1. **Don't be rude, even if you're right.**
    > Especially if you're right because being rude just makes you wrong in a whole different way. Being rude takes up energy and mental cycles that could be better spent on finding solutions. People are bound to make mistakes or offer bad suggestions. If there are sound, non-emotionally charged and non-biased reasons for rejecting an idea, please present them calmly and politely. If mistakes are made, attempt to address those directly to the person who made the mistake, privately if at all possible, and politely explain what the mistake was and how to correct it. Public humiliation will not be tolerated.
2. **Everyone has something of value to offer. Our job as a team is to find the nugget and give it a shine.**
    > I've worked with junior coders and non-devs who have offered up some of the most ingenious ideas which turned into exceptional solutions once time was taken to understand the thoughts. Everyone is busy. But we can make wonderful things happen if we slow down or even stop briefly once in a while to reflect on what has been done and politely discuss what we want to do. Doing so will allow us to proceed with confidence.
3. **Use common sense and kindness when coding, testing or supporting our work.**
    > When coding, if there is a common principle like how Rust has the concept of "idiomatic Rust", use it. If there is an "Angular way" to do a thing, that's the way we should be doing it. Often times, there are very, very good reasons for following these practices. This said, I am a bit of a stickler for the OTB (One-True Brace) style of coding for good reason, not just because I'm the product of learning to program in the late 80s / early 90s. However, I'm not going to go full-on Richard Hendricks (see the Silicon Valley show from HBO) over this OR spaces over tabs. Thankfully, that's because this is a Rust project and I already like the way `cargo fmt` does things.
    >
    >Being kind when coding refers to leaving quality code comments whenever possible, cleaning up debug statements and other little messes we often leave behind. This is just good project stewardship, good practice and very likely, kind to your future self. Not to mention someone else may come back to it at some point and they might just know where you live. :-)
4. **"WIP" is not an acceptable commit statement. Use conventional commit messages instead.**
    >First, all code projects are "work in progress" so WIP is a **Big Ol' Bucket-o-Duh:tm:**. Committing "work in progress" to a work in progress is redundant, useless and redundant. [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) are a clean, standard way to provide all the information necessary to convey. Let's use those.
5. **No man is an island and no one knows everything.**
    >Individually, we may be pretty smart. Cooperatively, we are freaking geniuses. I know enough to be very dangerous, but there are certainly things I don't know (yet) and my approach may not always be best. That even covers the first few points here. If there are things we can do better and should document them, then by all means let's discuss them and get things set in place.

### Project Goals<a id="goals"></a>

I try to outline project goals very clearly at the start of every thing I or my teams have done. We may not have all the requirements or designs right away, but we do have some basic boundaries set in order to reduce, if not eliminate, scope creep. In the case of this project, I have defined the goals in the [README.md](./README.md#goals). Let's do our best to color within the lines and if necessary, move additional features or design ideas to a whole new page, i.e. a project of its own. Can you tell I've had kids?

### How can I contribute?<a id="howcani"></a>

Anyone can contribute at anytime in any or all of the following ways:

- [By Reporting Bugs](#bugs)
- [By Suggesting Enhancements](#enhancements)
- [By Providing Code Improvements](#code)
- [By Building and / or Running Tests](#tests)
- [By Improving Documentation](#documentation)

#### By Reporting Bugs<a id="bugs"></a>

Please, by all means, if you use this library and it doesn't work then let us know by submitting an issue.

When doing so, please include the operating system info (name, version), Rust version, any error message that was produced and a detailed explanation of what is wrong and what is expected. I (for now), will do my best to address it in a timely fashion.

---

#### By Suggesting Enhancements<a id="enhancements"></a>

If you can think of some way to make this project better and it falls within the boundaries of the project goals, please submit a request for an enhancement and I (for now) will do my best to answer any questions or see it included in the project.

---

#### By Providing Code Improvements<a id="code"></a>

##### 1. Your first code contribution

First, this library was originally developed on an Arch Linux setup but it should work anywhere Rust does. There shouldn't be any OS specific things at work here.

Secondly, there are already more dependencies than I would like to have in the cargo.toml. So if you're code contribution includes a new dependency, please explain in detail why the dependency is needed and how the project benefits.

All that being said, code contributions are always welcome. Please follow best practices by creating a new branch locally, make your changes, test them and submit a pull request. Be sure to:

1. Comment / document your code as completely as possible using standard conventions.
2. Include tests for your code, if possible.
    - If not possible, please add a skeleton function for testing your function with an explanation of what conditions should be tested and what the expectations are for those tests.
3. Provide example usage for your changes, if possible, in code comments.
4. Clean up any debug mess.
5. Ensure that all existing tests pass.

##### 2. Do it the Idiomatic Rust way

When coding, do it the Idiomatic Rust way. That being - name your variables appropriately (duh), use `cargo fmt` to clean up code structure, use `cargo clippy` to test your syntax and code's buildable state, update your documentation with `cargo doc`, etc. There are numerous sources for how to be idiomatic, primarily the [Rust book](https://www.rust-lang.org/learn). That's about all I can say on this topic for now.

##### 3. Pull requests

When submitting a pull request, please be sure that you have pushed as clean a commit as possible to the repo and give your PR a simple but descriptive name. It will be reviewed for suitability and functionality as quickly as possible and merged or sent back with documented reasons. No PR will be rejected without reason. This seems about as simple a process as can be expected for now. I'm sure this will get more formalized as time progresses.

---

#### By Building and / or Running Tests<a id="tests"></a>

Testing any application is a necessary... er... evil? It's critical in any case. Testing should be as complete as possible and should test at least one functional path for each method or function in the code at a minimum. This, of course is not always possible. In those cases, the function should be documented as "untestable" but I would expect this to be rarely, if ever, used.

I have implemented `tarpaulin` for gathering code coverage information from tests. It probably needs more tweaks in the configuration, but I got it working fine for now. Anyone with better experience with `tarpaulin` is certainly welcome to help with that.

##### Your first test contribution

Just like the section above, [By Providing Code Improvements](#code), tests should be written in a new development branch locally and then submitted to the repo for a PR. Since Rust tests are usually bundled with the code and is actual Rust code, the same coding standards apply. In other words...

##### Do it the Idiomatic Rust way

This is the same as the section of the same name under [By Providing Code Improvements](#code). Use `cargo fmt` to format your code. Use `cargo clippy` to verify the syntax. Use `cargo doc` to update any documentation.

##### Pull requests

Again, very much like the similarly named section under [By Providing Code Improvements](#code). Be descriptive in your PR title and description. Make sure your PR commit is as clean as possible.

---

#### By Improving Documentation<a id="documentation"></a>

Anyone who likes to document things and would like to contribute, please do. Documentation. Blegh. I've always been decent at documentation. Over the many years in this industry, I've worn a lot of hats including systems analyst and technical writer. Those were the two roles I disliked most. And as you can see from this document, the README and possibly even the code documentation, I'm probably ok at it but after a while the quality begins to degrade. Case in point...

##### In Code: :thinking: Do it the Idiomatic Rust way?

Documenting code in Rust is actually pretty nice. And the fact that clippy will extract the important bits from the code comments and make it pretty makes it much better in my opinion. That said, there is an idiomatic way to document code in Rust and that's the way we should probably do it. The lazy answer is to follow the recommendations in the [Rust Book](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html?highlight=comment#making-useful-documentation-comments).

##### General: Do it the Idiomatic Markdown way... wait a minute

See what I mean about my documentation degrading after a while? Anyway, along with documenting the code which is necessary in the crates.io pages, there are also other documentation needs for this project. Namely the README.md, this file and potentially more detailed documentation elsewhere within the repo. The easiest way to maintain a standard presentation is with Markdown, so we'll use that for now. Markdown Masters with a skill for smithing words, please make your way to the front of the line.

---

### That's all for now

So we've reached the end of this rant. If you've made it this far, you're probably about as sick in the head as I am. In that case, nice to meet you. I would like to thank you again for even considering a contribution to this project and look forward to making this library awesome - together. Now, go forth and conquer and may the force be with you.
