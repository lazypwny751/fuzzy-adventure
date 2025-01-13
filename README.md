# rusttale
Help me to learn this technology with making some project, there are used technologies:
- rust
- bash
- clippy
- tetra
- rust localize

## TODO:
- localize CI/CD scripts.

## Localization support.
There's a limited localization support, that's not limiting the language entry or context lenght but there is no variables or any custom values, so every entry must be constant, and on the another side, creating this transcription is too easy.

You must look up the "shell" files under the "locales" directory, for example: [part-one](locales/part-one.sh).
for add new entries you can create a new shell file and also you can add context like the format:
```sh
# 		title
title "hello world"
	#	language  context
	entry "en" "Hello World!"
	entry "tr" "Merhaba Dünya!"
	entry "fr" "Bonjour le Monde!"
end
```
this will generate an transcription file as rust.

### Change language.
Default language is english.
```sh
cargo run -- --language <language>
# or
rusttale --language <language>
```

## Sound&Music.
- [supercollider](https://supercollider.github.io)

# Contributing.
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License
[MIT](https://choosealicense.com/licenses/mit/)
