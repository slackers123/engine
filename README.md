# engine

This is the start of a rust based game and possibly rendering engine.
Some of the planned features can be found in the [Ideas](Ideas.md) file.

<br><br>

## How to use
The project currently consist of 3 main cargo crates: entry_point, sandbox, tarator

<br>

> **tarator**<br>
> is the engine API. Here, every core functionality is defined and implemented.

<br>

> **sandbox**<br>
> is the application. It's main struct, application, is where the tarator API get's used to create the application. It's more of a developing test enviroment (that's why it's called "sandbox"), so the engine application witll probably get it's own crate, or this crate will get renamed.

<br>

> **entry_point**<br>
> is where the main-function is nested. Here, everything from the above 2 gets initialized (logger, windows, applications, etc.)

<br><br>

other crates are more or less for testing purposes