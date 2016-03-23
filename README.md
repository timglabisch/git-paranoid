# git-paranoid

you dont trust your coworkers and want to recognize as fast as possible if they are doing weird stuff in your project?
git-paranoid will help you to supervise every commit of any of your co-workers.

git-paranoid allows you to run simple checks against every commit in every branch on your repository.
the idea is that you can run simple regexs to find weird commits as fast as possible.

for example you can use git-paranoid to recognize any commit in any branch for a bunch of files.
this can be very helpful if you really want to make sure that nobody is changing your genius code without your approval.

or may you don't want your co-workers to use some kind methods - no problem, just write a small regex and find every possible violation.

## installation

clone the repo and use cargo install

```
git clone https://github.com/timglabisch/git-paranoid.git
cd git-paranoid
cargo install
```

## Example Configuration

create a *paranoid.toml* in your root directory and run *git-paranoid*.
right now every field is required and a regex.
code, ignore_authors and ignore_branches are arrays.

```
[dont_use_teardown_or_setup]
path = 'test'
code = ['tearDown', 'setUp']
ignore_authors = []
ignore_branches = []

[take_care_with_dateTime]
path = '.*'
code = ['new \\DateTime\(\)']
ignore_authors = []
ignore_branches = []

[category]
path = 'Category'
code = ['.*']
ignore_authors = []
ignore_branches = []
```

git-paranoid is alpha, don't expect too much for now.
