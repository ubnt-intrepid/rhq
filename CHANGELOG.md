# 0.2.4
* __Added:__   subcommand `rhq add` to add existed repository under management
* __Added:__   option `--depth` to restrict the depth of base directories
* __Deleted:__ option `--ssh` from `rhq new`

# 0.2.3
* Enforce to use SCP if protocol is SSH
* __Deleted:__ relative path with hostname (e.g. `github.com/ubnt-intrepid/peco`)
* __Added:__   support for cache file and subcommand `rhq scan`

# 0.2.2
* __Added:__ support for SSH protocol (#6)

# 0.2.1
* Accepts SCP-like patterns with suffix `git` (#2)
* `rhq clone`: add option `--root` for setting root directory explicitly
* Add subcommand: `rhq new`
* Start distributing the prebuilt binaries
