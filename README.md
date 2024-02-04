```
Generate colorful prompts for bash.

## Generate the config:
psprompt --generate > ~/.config/psprompt.toml

## Apply to the current session only:
eval $(psprompt)

## Apply to new sessions:
echo 'eval $(psprompt)' >> ~/.bashrc

Usage: psprompt [OPTIONS]

Options:
      --config <CONFIG>
          Config file (overriden by command line arguments)

      --generate
          Print template config file

  -s, --style <STYLE>
          Style
          
          [possible values: double, extended, simple, small, micro, nano]

  -U, --user <USER>
          User name

  -H, --host <HOST>
          Host name

  -L, --location <LOCATION>
          Location

  -I, --icon <ICON>
          Icon

  -h, --help
          Print help (see a summary with '-h')
```
