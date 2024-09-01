# Spammy-rs

<div align="center">
<p><strong>English</strong> | <a href="README_es.md">Spanish</a></p>
</div>

## Beginning

Hello! This is a small spammer written in Rust, follow the recommended network setup with ngrok [here](https://discord.com/developers/docs/tutorials/developing-a-user-installable-app#set-up-a-public-endpoint), ignore everything else.

The app ID is brought from the token and the public key can be too, just I'm too lazy to do that rn.

Soon the message will be provided in the same command, stay tuned here!

## Configuration

First of all, create an application in [discord developer portal](https://discord.com/developers/applications), go to general information and copy public key and paste it to your .env file.

Then go to installation and uncheck guild install, save changes and copy the install link provided by discord, this will prompt you to install the app in your account.

After that, go to bot page and click reset token, copy the token discord gave you and put it into the .env file too.

We're done with the discord variables, everything else is done by hand and is explained below.

See the example at [.env.sample](.env.sample), you must set all the values there, if you don't, the executable will panic showing you the missing variables, the most trivial are the listening port and command name, but you still need to set them properly.

The only one with special requirements is `RESPONSE`, this variable is a hex-represented utf-8 string, you can convert any utf-8 text you want to hex with [this url](https://cyberchef.io/#recipe=To_Hex('None',0)), this is the string that the bot will respond when you execute the command.

## Compilation

First, you need to install Rust with [rustup](https://www.rust-lang.org/tools/install) or a package provided by your linux distro, for Windows, follow the steps guided by the installer.

Before running anything, run `loadenv.sh` in linux and `loadenv.bat` in windows to load .env file to your current shell environment, and everytime you update .env file, ensure you run loadenv after updating it.

Then, after checking if rust is in your path, execute `cargo run` to automatically download dependencies, build and run the listener.

### Disclaimer

This is done purely for educational purposes, the code and software is provided as is without warranties or any kind of responsibility for the usage as described in the [license](LICENSE).
