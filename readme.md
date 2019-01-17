# dotfile

A fast, lightweight, and extensible dotfile manager and post-install installer
for Arch users. Use `dotfile` as a simple way to manage dotfiles anywhere and
a way for users to quickly get back on their feet after a fresh installation.

## Features

- Multiple dotfile configuration support
- Backwards compatible with `stow`
- Pre- and Post-run scripting support (*Planned*, v1.1)
- Semi-automated initialization and configuration
- AUR helper support

## Installation

Arch users, please use an AUR helper to install `dotfile`.

```bash
trizen -Syu dotfile
```

Please refer to the ArchWiki for installing `dotfile` manually.


## Usage

`dotfile` has many features, so we'll first present some common use cases for
users to quickly get up to speed.

### Getting started with `dotfile`

```bash
# Initializing a new dotfile repository
dotfile init

# Adapting from a stow-managed dotfile. This also sets the default dotfile
# repository location. Converts the stow-managed repo into a dotfile repo.
dotfile init --from /path/to/stow/repo

# Initializing a new dotfile repository at a custom path
dotfile init /custom/path
```

### Configuring `dotfile`
```bash
# Set trizen as the AUR helper
# The AUR helper optimally supports pacman-like syntax
dotfile use --helper trizen

# Clears the AUR helper
dotfile use --helper

# Set default dotfile repository location
dotfile use --path /path/of/repo

# Reset the default dotfile repository to ~/dotfiles
dotfile use --path

## Complex examples

# You can simutanously set up multiple configs as well
dotfile use --path /foo/bar --helper yay
```

### Adding packages
`dotfile` is package-based, not program name-based. If you have an application
called `foobar` but the package is called `foo` (e.g. `sudo pacman -Syu foo`),
then you should add `foo` instead of `foobar`. This lets `dotfile` act as an
personalized installer if you ever have to reinstall arch from scratch.
```bash
# Populates the dotfile folder with explicitly installed packages
# (Runs pacman -Qqet). Used if this is your first time a dotfile repo.
dotfile add --populate

# Adds mpv to the default group, and attempts to stow away its config folder in
# ~/.config
dotfile add mpv

# Adds mpv, chromium, and vscode to the default group
dotfile add --packages mpv chromium vscode

# Adds mpv to the dotfile group "group1", and attempts to stow away its config
# folder in ~/.config
dotfile add mpv --groups group1

# Adds mpv to multiple groups, and attempts to stow away its config folder.
dotfile add mpv --groups group1 group2

# Adds mpv and vscode to multiple groups, and attempts to stow away the config
# folders
dtofile add --packages mpv vscode --group group1 group2

# Adds mpv to the default group with a specified config folder
dotfile add mpv --config /path/to/config/dir

# Adds some Xorg config files and folders. If -c is specified, dotfile will not
# attempt to look for a directory under the same name under ~/.config
# -c is equivalent to --config
dotfile add -c ~/.config/Xorg/.icons/ ~/.Xresources ~/.xinitrc -- x

# Adds mpv without trying to stow away a config folder
dotfile add --no-config mpv


## Complex examples

# Adds mpv to a nondefault dotfile path and into their group2
dotfile add --path /path/to/other/dotfile/path --group group2 mpv
```

### Removing packages
```bash
# Removes mpv from the default group, and unstows the config directory
dotfile remove mpv

# Removes mpv, chromium, and vscode from the default group
dotfile remove mpv chromium vscode

# Removes mpv from the default group but keeps the config directory
dotfile remove --keep-config mpv

# Removes mpv from only group1
dotfile remove --group group1 mpv

# Removes mpv from the specified dotfile path and unstows the config directory
dotfile remove --config /path/to/other/dotfile/path mpv
```

### Group management
`dotfile` has the concepts of groups to help manage dotfile installations. You
can either think of them like mathematical sets that you can union together, or
for specific installation setups (e.g. for a laptop or desktop).

```bash
# Creates the group "group1" in the default dotfile path
dotfile group create group1

# Deletes the group "group1" in the default dotfile path
dotfile group rm group1

# Renames the group "group1" to "group2"
dotfile group mv group1 group2
```

### Installing packages
This feature is meant for fresh installs of Arch linux, but may be used in other
situations. The `install` subcommand lets you quickly install groups of packages
and allows modular control of packages to install.

```bash
# Install only the common group of packages
dotfile install

# Install packages in common, group1, and group2
dotfile install group1 group2

# Install packages in group1 and group2 only
dotfile install --no-common group1 group2

# Install packages in common, group1 and group2, but do not run
# scripts found in group2
dotfile install group1 --no-script group2 # dotfile install group1 -n group2

# Do not install any scripts
dotfile install --no-scripts group1 group2 # dotfile install -N group1 group2
```

### Scripting

`dotfile` allows for scripting for extensible install scripts before and after
a group has been installed. Simply have `pre.sh` and `post.sh` in the group
folder and `dotfile` will run the respective scripts at the correct time.
```
dotfiles
├─common          Common group will run pre.sh and post.sh
│ ├─pre.sh
│ ├─mpv
│ │ └─.config
│ │   └─mpv
│ │     └┄
│ └─post.sh
│
├─laptop          Laptop group will only run a pre-start script.
│ ├─pre.sh
│ └─polybar
│   └─.config
│     └─polybar
│       └┄
└─desktop         Desktop group will only run a post-complete script.
  ├─zsh
  │ └─.config
  │   └─zsh
  │     └┄
  └─post.sh
```

### Using `dotfile` as a post-install setup.

You have just installed Arch and created a user. You would like `dotfile` to install your dotfiles from `git`.

#### Prerequisites
 - `git`
 - `sudo` You **must** be a sudoer.
 - `ssh` (*Optional: You can git clone via HTTPS instead.*)

```bash
dotfile clone git@github.com:username/repository ~/dotfiles

# Will install only the common dotfiles (all dotfiles if you do not have
# multiple dotfile groups), as well as the packages. Runs all common scripts.
dotfile install

# Install common dotfiles and dotfiles from group1, group2, and group3, as well
# as their packages.
# Runs scripts from common, group1, group2, and group3.
dotfile install group1 group2 group3

# Installs only group1 dotfiles. Does not install common dotfiles.
# Runs scripts only from group1.
dotfile install --no-common group1 # or dotfile stow -n group1
```
