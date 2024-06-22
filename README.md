# Jumb

How many times do you run the `cd` command in a day? If you're like many developers, it's probably more than you'd like to count.

![Friendship ended with X, now Y is my best friend](https://github.com/ChetanXpro/jump/assets/107798155/cfece286-e927-4616-b73d-0e68afa65df8)


**Jumb** is a command-line tool designed to dramatically enhance directory navigation. By allowing users to create and use shortcuts for frequently accessed directories, Jumb reduces the need to execute lengthy `cd` commands. This not only streamlines your workflow but also saves a significant amount of time throughout your day.


## Key Benefits

- **Reduce `cd` Usage**: Minimize the need to use lengthy `cd` commands by setting up shortcuts to your most used directories.
- **Save Time**: Dramatically reduce the time spent navigating between different parts of your file system, enhancing productivity, especially for developers and system administrators.
- **Streamline Workflow**: Improve your workflow efficiency by quickly switching contexts between projects and directories.

## Installation

### Using Cargo

Install Jumb directly from Cargo:

```bash
cargo install jumb
```

- NOTE: after intalling jumb with cargo , wrapping it with shell is must, to make the tool work properly.
### Setting Up the Shell Wrapper

Integrate Jumb with your shell by adding a shell wrapper to your .zshrc or .bashrc file. This wrapper ensures that using Jumb feels like a natural extension of your shell commands.

```bash
# Add this script to your shell configuration file (.zshrc or .bashrc)

function jnav() {
    # Ensure no conflicts with existing 'jumb' commands
    unalias jumb 2>/dev/null
    unset -f jumb 2>/dev/null

    # Locate the Jumb binary dynamically
    local jumb_path=$(which jumb)
    if [[ -z "$jumb_path" ]]; then
        echo "Jumb is not installed or not found in PATH."
        return 1
    fi

    # Execute Jumb commands
    if [[ "$1" == "view" ]] && [[ $# -eq 1 ]]; then
        $jumb_path view  # Lists directories without changing the directory
    elif [[ $# -eq 1 ]]; then
        local dir=$($jumb_path "$1")
        if [[ -d "$dir" ]]; then
            echo "Changing directory to: $dir"
            cd "$dir"
        else
            echo "Directory not found: $dir"
        fi
    else
        $jumb_path "$@"
    fi
}

```

### Customizing Function Name

If you prefer a different name for the function, replace jnav with your chosen name to prevent potential conflicts with the Jumb binary.


## Usage

Set up and manage your directory shortcuts easily:

- Add a Shortcut

```bash
jnav add <shortcut_name> /path/to/directory
```

For example if you want to add current dir path then you can run command with a full stop to add current dir.
```bash
jnav add project .
```

- Remove a Shortcut

```bash
jnav remove shortcut_name
```


- View All Shortcuts

```bash
jnav view
```

- View particular shortcut path

```bash
jnav view <Shortcut_name>
```


Navigate quickly using:

```bash
jnav shortcut_name
```


## Contributing
Contributions are encouraged. Feel free to fork the project and submit pull requests.
