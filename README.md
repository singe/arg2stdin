# arg2stdin
A simple utility to pass cli arguments as stdin to a specified command.

# Usage

`arg2stdin <command> <string to pass to stdin>`

For example:

`arg2stdin rev hello`

The command doesn't need to be a single command, and can be a long shell one liner too. For example:

`arg2stdin 'sed "s/A/X/g" | grep -o "X" | wc' AAAAAAAA`

# The Problem

Sometimes you don't want to pass something via stdin, but the command only accepts stdin. For example if I wanted to print the string 'hello' backwards I might try:

`rev hello`

This will fail because there is no file named 'hello'. Instead, we're supposed to:

`echo hello | rev`

Wouldn't it be nice to be able to do this instead:

`arg2stdin rev hello`

Well, ok, it's longer than using echo and pipes, but sometimes you're executing in a way that doesn't give you access to stdin, then it's useful.

The alternative would be something like this (thanks @vhata):

`bash -c 'echo "$0 $@" | rev' hello`

In which case `arg2stdin` makes it a little more intuitive.
