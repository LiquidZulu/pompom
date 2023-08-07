![img](./pompom-demo.gif)


# Table of Contents

1.  [installation](#org6262629)
    1.  [build from source](#org2a08df8)
    2.  [crates.io](#org06c6506)
2.  [usage](#org63d2c98)
3.  [configuration](#org1eaa132)
    1.  [default config](#orgd9c9e12)
    2.  [splash screen variant](#orgc032d9f)
    3.  [schedule](#orgd63dccb)
    4.  [duration](#org5d2094f)


<a id="org6262629"></a>

# installation


<a id="org2a08df8"></a>

## build from source

The source code can be found at [this](https://github.com/LiquidZulu/pompom) GitHub repo. Nix tooling is provided through `default.nix` and can be accessed by running `nix-shell` after navigating to the repository on your terminal. I do not believe this step to be necessary on non-NixOS distributions, all that this does is provides [ALSA](https://alsa-project.org/wiki/Main_Page) which is required to compile.

When the source has been obtained the easiest way to compile is with `cargo build --release`, then add `path/to/pompom/target/release` to your [PATH](https://en.wikipedia.org/wiki/PATH_(variable)). I do not know if this software compiles on Windows, if you have any problems with doing this [open an issue](https://github.com/LiquidZulu/pompom/issues).


<a id="org06c6506"></a>

## crates.io

This software is distributed also at [crates.io](https://crates.io/crates/pompom), and should be able to be installed with `cargo install pompom`.


<a id="org63d2c98"></a>

# usage

    ~ » pompom --help
    Usage: pompom [OPTIONS] [WORK_DURATION] [REST_DURATION] [LONG_REST_DURATION]
    
    Arguments:
      [WORK_DURATION]
      [REST_DURATION]
      [LONG_REST_DURATION]
    
    Options:
      -u, --unit <UNIT>  [default: minutes] [possible values: seconds, minutes, hours]
      -h, --help         Print help
      -V, --version      Print version


<a id="org1eaa132"></a>

# configuration


<a id="orgd9c9e12"></a>

## default config

The following default configuration file should be generated at `$XDG_CONFIG_HOME/pompom/config.toml` upon the first time running pompom:

    splash_screen_variant = "Row"
    schedule = ["Work", "Rest", "Work", "Rest", "Work", "Rest", "Work", "LongRest"]
    
    [work_duration]
    Minutes = 25
    
    [rest_duration]
    Minutes = 5
    
    [long_rest_duration]
    Minutes = 30


<a id="orgc032d9f"></a>

## splash screen variant

Options for `splash_screen_variant` are provided by the `SplashScreen` enum in `src/splash_screen.rs`, they are:

-   Row
-   Stacked
-   None

Any text other than &ldquo;Row&rdquo; or &ldquo;Stacked&rdquo; will be interpreted as disabling the splash screen.


<a id="orgd63dccb"></a>

## schedule

`schedule` defines the pomodoro loop; so if you have `["Work", "Rest", "Work", "LongRest"]`, this would define a loop with a work period, followed by a rest period, followed by another work period, then a long rest before starting back at the first work. The default loop is 4 cycles of work/rest followed by a long rest. Valid periods are defined by `Period` in `src/types.rs`.


<a id="org5d2094f"></a>

## duration

The `[*_duration]` variables define how long each period should take, valid units are defined by `Duration` in `src/types.rs`. So if you wanted the long rest period to be 1 hour you could achieve this with the following configuration:

    [long_rest_duration]
    Hours = 1

