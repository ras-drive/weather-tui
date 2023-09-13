#[derive(PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum WeatherCondition {
    Sunny,
    Cloudy,
    Raining,
    Thunder,
    Snowing,
}

impl WeatherCondition {
    // used to parse "current condition" field of backend api
    pub fn parse(condition: &str) -> Option<Self> {
        match condition.to_ascii_lowercase().as_str() {
            str if str.contains("sunny") || str.contains("clear") => Some(WeatherCondition::Sunny),
            str if str.contains("cloudy") || str.contains("overcast") => {
                Some(WeatherCondition::Cloudy)
            }
            str if str.contains("thunder") => Some(WeatherCondition::Thunder),
            str if str.contains("rain")
                || str.contains("mist")
                || str.contains("fog")
                || (str.contains("drizzle") && !str.contains("freezing")) =>
            {
                Some(WeatherCondition::Raining)
            }
            str if str.contains("snow")
                || str.contains("sleet")
                || str.contains("blizzard")
                || str.contains("ice")
                || str.contains("freezing") =>
            {
                Some(WeatherCondition::Snowing)
            }
            _ => None,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            WeatherCondition::Sunny => std::str::from_utf8(SUNNY).unwrap(),
            WeatherCondition::Cloudy => std::str::from_utf8(CLOUDS).unwrap(),
            WeatherCondition::Raining => std::str::from_utf8(RAINING).unwrap(),
            WeatherCondition::Thunder => std::str::from_utf8(LIGHTNING).unwrap(),
            WeatherCondition::Snowing => std::str::from_utf8(SNOW).unwrap(),
        }
    }
}

pub const SUNNY: &[u8] = br#"
      ;   :   ;
   .   \_,!,_/   ,
    `.,'     `.,'
     /         \
~ -- :         : -- ~
     \         /
    ,'`._   _.'`.
   '   / `!` \   `
      ;   :   ;  
"#;

pub const CLOUDS: &[u8] = br#"
   __    ___
 _(  )__(   )_
(_   _    ___)
  (_) (__) ( )_
   (_   _    _)
     (_) (__)
"#;

pub const RAINING: &[u8] = br#"
          __   _
        _(  )_( )_
       (_   _    _)
      / /(_) (__)
     / / / / / /
    / / / / / /
   / / / / / /
"#;

pub const LIGHTNING: &[u8] = br#"
     _, .--.
    (  / (  '-.
    .-=-.    ) -.
   /   (  .' .   \
   \ ( ' ,_) ) \_/
    (_ , /\  ,_/
      '--\ `\--`
         _\ _\
          _\_\
           `\\
        -.'.`\.'.-
"#;

pub const SNOW: &[u8] = br#"
          /\
     __   \/   __
     \_\_\/\/_/_/
       _\_\/_/_
      __/_/\_\__
     /_/ /\/\ \_\
          /\
          \/
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_condition_sunny() {
        assert_eq!(
            WeatherCondition::parse("Clear").unwrap(),
            WeatherCondition::Sunny
        )
    }

    #[test]
    fn test_weather_condition_cloudy() {
        assert_eq!(
            WeatherCondition::parse("Partly cloudy").unwrap(),
            WeatherCondition::Cloudy
        )
    }

    #[test]
    fn test_weather_condition_rainy() {
        assert_eq!(
            WeatherCondition::parse("Patchy light drizzle").unwrap(),
            WeatherCondition::Raining
        )
    }

    #[test]
    fn test_weather_condition_snowy() {
        assert_eq!(
            WeatherCondition::parse("Moderate or heavy sleet showers").unwrap(),
            WeatherCondition::Snowing
        )
    }

    #[test]
    fn test_weather_condition_thunder() {
        assert_eq!(
            WeatherCondition::parse("Patchy light snow with thunder").unwrap(),
            WeatherCondition::Thunder
        )
    }
}
