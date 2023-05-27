Work in progress

# git-config-manager

> Manage your git config like a pro

## About

This small tool allows you to have multiple git configurations. The main reason that I created this tool it wasn't to have multiple git configurations, but instead being able to store the configuration outside of the main `.gitconfig` file and being able to commit my global `.gitconfig` to my dotfiles repository.

## How does it work?

This is where your configuration will be stored.

```
~
├── .config
│   └── git-config-manager
│       ├── .gitconfig-main
│       ├── .gitconfig-personal
│       └── .gitconfig-work
├── .gitconfig
└── .vimrc
```

and your `.gitconfig` will look like this:

```ini
[include]
    path = ~/.config/git-config-manager/.gitconfig-main

[includeIf "gitdir:~/dev/personal/"]
    path = ~/.config/git-config-manager/.gitconfig-personal

[includeIf "gitdir:~/dev/work/"]
    path = ~/.config/git-config-manager/.gitconfig-work
```

To test that the correct profile is being used, navigate to one of your git repositories, for example:

```bash
cd ~/dev/personal/my-awesome-project
```

> Note: You must be inside a git repository, and the `icludeIf` path must end on a `/`.

and run:

```bash
git config --get user.email
```

You should get the email for that specific profile. Test it with multiple profiles.

## Usage

> This still in development, so the usage might change.

```bash
git-config-manager <path-to-config>
```

## Using 1Password

If you are using 1Password, you can inject the credentials using the `op` cli. Make sure you have the `op` cli installed and you replace the `op` path with your own.

```bash
op inject -i ./example/config.op.json -o ./example/config.json
```
