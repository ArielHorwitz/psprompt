```
Generate colorful prompts for the terminal.

## Generate the config:
psprompt --generate > ~/.config/psprompt.toml

## Apply to the current session only:
eval $(psprompt)

## Apply to new sessions:
echo 'eval $(psprompt)' >> ~/.bashrc

Usage: psprompt [OPTIONS]

Options:
      --config <CONFIG>
          Config file (default: ~/.config/psprompt.toml)

      --generate
          Print template config file

  -s, --style <STYLE>
          Style
          
          [possible values: double, extended, full, normal, micro, nano]

  -U, --user <USER>
          User name

  -H, --host <HOST>
          Host name

  -L, --location <LOCATION>
          Location

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
