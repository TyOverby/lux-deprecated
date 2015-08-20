This directory contains the content for the main Lux website.
It also contains scripts for compiling the content and publishing it to the gh-pages branch.

The `out` directory is actually a git submodule that links up to the gh-pages branch for *this* repo.
Human-written content should never be placed inside `out`; `out` is only for machine generated or
machine moved files.  For example, the documentation generation script throws the files directly
in `out`.

The `publish` script simply goes into that submodule, `git-add`s, `git-commit`s, and pushes.
