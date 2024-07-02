# Cargo Expand

1. To view the expanded macros, open a terminal, and run the following command.

    ```bash
    cargo expand --lib > temp.txt
    ```

2. Open the `temp.txt` file, and search for `<h1`. This will show you the expanded macro content for creating the `h1` element.
