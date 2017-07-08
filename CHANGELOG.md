# 0.2.7
* __(fixed)__   Correct destination path when HTTPS URL is given (#25)

# 0.2.6
* __(added)__   Support for exclude pattern at scanning (#20)
* __(fixed)__   Change command line interfaces
  - rename `rhq init` to `rhq new`
  - remove `rhq scan` and add option `--import` to `rhq add`
  - add `rhq refresh` to refresh existed entries
  - remove option `--dry-run` from `rhq new` and `rhq clone`
* __(fixed)__   Change format of cache file
  - TOML -> JSON
* __(fixed)__   Read remote URL from VCS after adding repositories
* __(added)__   option `--format` to `rhq list`

# 0.2.5
* __(fixed)__   `rhq new/clone` performs VCS command before creating instnance of `Repository` 
* __(added)__   Support for more DVCS (Mercurial, Darcs and Pijul)
* __(fixed)__   `rhq new` are replaced to `rhq init` and take specific target directory (instead of remote information)
* __(fixed)__   `rhq clone` takes an optional argument `[dest]` to specify the target directory of cloned repository
* __(fixed)__   `rhq add` allows to take multiple inputs

# 0.2.4
* __(added)__   subcommand `rhq add` to add existed repository under management
* __(added)__   option `--depth` to restrict the depth of base directories
* __(deleted)__ option `--ssh` from `rhq new`

# 0.2.3
* __(fixed)__   Enforce to use SCP if protocol is SSH
* __(deleted)__ Relative path with hostname (e.g. `github.com/ubnt-intrepid/peco`)
* __(added)__   Support for cache file and subcommand `rhq scan`

# 0.2.2
* __(added)__   Support for SSH protocol (#6)

# 0.2.1
* Accepts SCP-like patterns with suffix `git` (#2)
* `rhq clone`: add option `--root` for setting root directory explicitly
* Add subcommand: `rhq new`
* Start distributing the prebuilt binaries
