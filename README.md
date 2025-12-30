# Weather Application - A Terminal User Interface

The basic information regarding current weather can be found on the terminal using this very basic Rust Application.

## Basic Mode
If you want to see the current day's weather report, pass the command-line arguments with the city name, for example, Berlin, and then normal, like the following:

`cargo run -- Berlin normal`

## Detail Mode
If you want to see the weather report for the upcoming days as well, then pass the command-line arguments with city name and detail, like the following:

`cargo run -- Berlin detail`


## Printing of Current Weather
Some basic printing functions have been added to print the current weather in normal mode with some colors.

## Configuring the Key
Need to create an `.env` file and define a property called `application_key` from OpenWeatherMap API.
