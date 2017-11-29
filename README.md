# Gru

## Self hosting backups made easy

Gru is the server part of the *DespicableBackup* project.
Ever wondered how to properly backup your self-hosted services?
This is an attempt to solve this problem.

Gru will run on your server, allowing **minions** to register
and flag themselves as ready to receive a backup.

This project aims to provide:
* off-site storage
* multiple backups
* transparent backup system (rsync, duplicity, borg, etc)

## How does it work?

Gru will maintain a database of your **minions**.
A minion is a friendly computer accepting connections over ssh.

Let's say you host services for Alice, Bob and you.
Bob has a raspberry pi with enough storage to accept your backups and he wants to help.

However:
* Bob might not be willing to let his raspberry run 24/24
* Bob might not know its IP address

This is where gru can help: it provides an API to let the minions
register themselves.

This is how you can backup:
* create a token and give it to your friend
* your friend register its minion
* run `gru get <tokenid>` to get minion data
* use these data to backup with your favorite backup tool (rsync, duplicity, borg, etc)

## Getting started

You create a token for him to register on your server:

    gru create bob

While the server is running (`gru serve`, more details [below](#how-to-run)),
Bob can register its minion using the client:

    minion-backup register -c <config>

This will flag its machine as ready to accept backups.
On shutdown, Bob will execute (or a systemd unit):

    minion-backup unregister -c <config>

On your side, when you are ready to backup, just run:

    gru list

And you will obtain a list of minions (username, ip, port) ready to accept your data.

### Advantages
* Minions do not need to run 24/24
* Minions do not need to have a DNS set up

### Drawbacks
* If none of you minions are up, you can't backup on a remote (always keep a local backup too)
* Restoring your data requires to contact directly your minions owners

## How to run

Gru should always be running to accept minions registrations.
The easiest way to achieve this is to enable the systemd unit:

    systemctl enable gru

## How to build

Building Gru requires rust nightly, the easiest way to get up and running is via [rustup](https://www.rustup.rs/).
Then simply run:

    cargo build --release

*Note to maintainers*: you can specify configuration path at build time through the "GRU_CONFIG_PATH" environment variable.

### Package for debian

    cargo install cargo-deb
    cargo deb

## License

MIT, see COPYING for details.
