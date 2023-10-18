# Demo

![Alt text](test.png?raw=true "Demo Image")

## Description

fetches data from the weatherAPI.com api and stores it into an app that displays an ascii weather graphic and a brief description of the weather. I'm attempting to have a robust config section and will have it documented here.

I'm building this for practice into building better software and have been trying to think more about design. This is by no means the best way to do this as no way is the best way, but it's my attempt at trying to make it correct. I tried to think about this project in a bit of a more functional manner.

I started building this while reading *"The Pragmatic Programmer"* and I think it helped influence me to pass values through functions more and use less of an imperative style.

### Config

The config file is a toml file located in the project directory for now, later I plan on using xdg directories to give it a config dir.

|     Key     |         Description        |
| ----------- | -------------------------- |
|  api_key    |     WeatherAPI.com key     |
|  location   | city, region("chicago il") |
