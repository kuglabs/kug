# Coding Guidelines

The goal of these guidelines is to improve developer productivity by allowing
developers to jump into any file in the codebase and not need to adapt to
inconsistencies in how the code is written. The codebase should appear as if it
had been authored by a single developer. If you don't agree with a convention,
submit a PR patching this document and let's discuss! Once the PR is accepted,
_all_ code should be updated as soon as possible to reflect the new
conventions.

## Pull Requests

Small, frequent PRs are much preferred to large, infrequent ones. A large PR is
difficult to review, can block others from making progress, and can quickly get
its author into "rebase hell". A large PR oftentimes arises when one change
requires another, which requires another, and then another. When you notice
those dependencies, put the fix into a commit of its own, then checkout a new
branch, and cherry-pick it.

```bash
$ git commit -am "Fix foo, needed by bar"
$ git checkout main
$ git checkout -b fix-foo
$ git cherry-pick fix-bar
$ git push --set-upstream origin fix-foo
```

Open a PR to start the review process and then jump back to your original
branch to keep making progress. Consider rebasing to make your fix the first
commit:

```bash
$ git checkout fix-bar
$ git rebase -i main <Move fix-foo to top>
```

Once the commit is merged, rebase the original branch to purge the
cherry-picked commit:

```bash
$ git pull --rebase upstream main
```

## Getting Pull Requests Merged

There is no single person assigned to watching GitHub PR queue and ushering you
through the process. Typically, you will ask the person that wrote a component
to review changes to it. You can find the author using `git blame` or asking on
Discord. When working to get your PR merged, it's most important to understand
that changing the code is your priority and not necessarily a priority of the
person you need an approval from. Also, while you may interact the most with
the component author, you should aim to be inclusive of others. Providing a
detailed problem description is the most effective means of engaging both the
component author and other potentially interested parties.

Consider opening all PRs as Draft Pull Requests first. Using a draft PR allows
you to kickstart the CI automation, which typically takes between 10 and 30
minutes to execute. Use that time to write a detailed problem description. Once
the description is written and CI succeeds, click the "Ready to Review" button
and add reviewers. Adding reviewers before CI succeeds is a fast path to losing
reviewer engagement. Not only will they be notified and see the PR is not yet
ready for them, they will also be bombarded with additional notifications
each time you push a commit to get past CI or until they "mute" the PR. Once
muted, you'll need to reach out over some other medium, such as Discord, to
request they have another look. When you use draft PRs, no notifications are
sent when you push commits and edit the PR description. Use draft PRs
liberally. Don't bug the humans until you have gotten past the bots.
