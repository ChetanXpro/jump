# Jumb

Jumb is a command-line tool that simplifies navigation across your file system by allowing you to create and use shortcuts for your directories. Instead of repeatedly typing long `cd` commands to access frequently visited directories, you can set up memorable shortcuts once and use them to jump directly to those directories.

## Key Benefits

- **Reduce `cd` Usage**: Minimize the need to use lengthy `cd` commands by setting up shortcuts to your most used directories.
- **Save Time**: Dramatically reduce the time spent navigating between different parts of your file system, enhancing productivity, especially for developers and system administrators.
- **Streamline Workflow**: Improve your workflow efficiency by quickly switching contexts between projects and directories.

## Installation

### Using Cargo

Install Jumb directly from Cargo, which handles the setup automatically:

```bash
cargo install jumb
```
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
