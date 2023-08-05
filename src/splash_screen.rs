use colored::Colorize;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub enum SplashScreen {
    Row,
    Stacked,
    None,
}

impl FromStr for SplashScreen {
    type Err = ();

    fn from_str(s: &str) -> Result<SplashScreen, ()> {
        match s {
            "Row" => Ok(SplashScreen::Row),
            "Stacked" => Ok(SplashScreen::Stacked),
            _ => Ok(SplashScreen::None),
        }
    }
}

pub fn splash_screen(ss_type: &SplashScreen) {
    match ss_type {
        SplashScreen::None => return,
        SplashScreen::Row => println!(
            "
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
                                                                         ",
            "                              ____  ".magenta(),
            "                              ____".cyan(),
            ",-.----.                    ,'  , `.".magenta(),
            ",-.----.                    ,'  , `.".cyan(),
            "\\    /  \\    ,---.       ,-+-,.' _ |".magenta(),
            "\\    /  \\    ,---.       ,-+-,.' _ |".cyan(),
            "|   :    |  '   ,'\\   ,-+-. ;   , ||".magenta(),
            "|   :    |  '   ,'\\   ,-+-. ;   , ||".cyan(),
            "|   | .\\ : /   /   | ,--.'|'   |  ||".magenta(),
            "|   | .\\ : /   /   | ,--.'|'   |  ||".cyan(),
            ".   : |: |.   ; ,. :|   |  ,', |  |,".magenta(),
            ".   : |: |.   ; ,. :|   |  ,', |  |,".cyan(),
            "|   |  \\ :'   | |: :|   | /  | |--' ".magenta(),
            "|   |  \\ :'   | |: :|   | /  | |--'".cyan(),
            "|   : .  |'   | .; :|   : |  | ,    ".magenta(),
            "|   : .  |'   | .; :|   : |  | ,".cyan(),
            ":     |`-'|   :    ||   : |  |/     ".magenta(),
            ":     |`-'|   :    ||   : |  |/".cyan(),
            ":   : :    \\   \\  / |   | |`-'      ".magenta(),
            ":   : :    \\   \\  / |   | |`-'".cyan(),
            "|   | :     `----'  |   ;/          ".magenta(),
            "|   | :     `----'  |   ;/".cyan(),
            "`---'.|             '---'           ".magenta(),
            "`---'.|             '---'".cyan(),
            "  `---`                               ".magenta(),
            "`---`".cyan(),
        ),
        SplashScreen::Stacked => println!(
            "
{}{}
                                                                         ",
            "
                              ____
,-.----.                    ,'  , `.
\\    /  \\    ,---.       ,-+-,.' _ |
|   :    |  '   ,'\\   ,-+-. ;   , ||
|   | .\\ : /   /   | ,--.'|'   |  ||
.   : |: |.   ; ,. :|   |  ,', |  |,
|   |  \\ :'   | |: :|   | /  | |--'
|   : .  |'   | .; :|   : |  | ,
:     |`-'|   :    ||   : |  |/
:   : :    \\   \\  / |   | |`-'
|   | :     `----'  |   ;/
`---'.|             '---'
  `---`"
                .magenta(),
            "                       ____
,-.----.                    ,'  , `.
\\    /  \\    ,---.       ,-+-,.' _ |
|   :    |  '   ,'\\   ,-+-. ;   , ||
|   | .\\ : /   /   | ,--.'|'   |  ||
.   : |: |.   ; ,. :|   |  ,', |  |,
|   |  \\ :'   | |: :|   | /  | |--'
|   : .  |'   | .; :|   : |  | ,
:     |`-'|   :    ||   : |  |/
:   : :    \\   \\  / |   | |`-'
|   | :     `----'  |   ;/
`---'.|             '---'
  `---`                               "
                .cyan(),
        ),
    };
}
